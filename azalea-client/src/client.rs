use crate::{
    chat::ChatPlugin,
    disconnect::DisconnectPlugin,
    local_player::{
        death_event, handle_send_packet_event, update_in_loaded_chunk, GameProfileComponent,
        LocalPlayer, PhysicsState, SendPacketEvent,
    },
    movement::{local_player_ai_step, send_position, sprint_listener, walk_listener},
    packet_handling::{PacketHandlerPlugin, PacketReceiver},
    player::retroactively_add_game_profile_component,
    runner::RunnerPlugin,
    task_pool::TaskPoolPlugin,
    Account, StartSprintEvent, StartWalkEvent,
};

use azalea_chat::FormattedText;
use azalea_ecs::{
    app::{App, Plugin, PluginGroup, PluginGroupBuilder},
    bundle::Bundle,
    entity::Entity,
    schedule::{IntoSystemDescriptor, Stage, SystemSet},
    system::Resource,
    AppTickExt,
};
use azalea_ecs::{ecs::Ecs, TickPlugin};
use azalea_physics::PhysicsPlugin;
use azalea_protocol::{
    connect::ConnectionError,
    packets::game::serverbound_client_information_packet::ServerboundClientInformationPacket,
    resolver, ServerAddress,
};
use azalea_world::{
    entity::{EntityPlugin, Local},
    WorldContainer,
};
use derive_more::{Deref, DerefMut};
use futures::channel::mpsc::UnboundedSender;
use log::error;
use parking_lot::Mutex;
use std::{fmt::Debug, io, sync::Arc, time::Duration};
use thiserror::Error;
use tokio::{
    sync::mpsc,
    time::{self, interval},
};

pub type ClientInformation = ServerboundClientInformationPacket;

/// An error that happened while joining the server.
#[derive(Error, Debug)]
pub enum JoinError {
    #[error("{0}")]
    Resolver(#[from] resolver::ResolverError),
    #[error("{0}")]
    Connection(#[from] ConnectionError),
    #[error("{0}")]
    ReadPacket(#[from] Box<azalea_protocol::read::ReadPacketError>),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    SessionServer(#[from] azalea_auth::sessionserver::ClientSessionServerError),
    #[error("The given address could not be parsed into a ServerAddress")]
    InvalidAddress,
    #[error("Couldn't refresh access token: {0}")]
    Auth(#[from] azalea_auth::AuthError),
    #[error("Disconnected: {reason}")]
    Disconnect { reason: FormattedText },
}

/// Connect to a Minecraft server.
///
/// To change the render distance and other settings, use
/// [`Client::set_client_information`]. To watch for events like packets
/// sent by the server, use the `rx` variable this function returns.
///
/// # Examples
///
/// ```rust,no_run
/// use azalea_client::{Client, Account};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let account = Account::offline("bot");
///     let (client, rx) = Client::join(&account, "localhost").await?;
///     client.chat("Hello, world!");
///     client.disconnect();
///     Ok(())
/// }
/// ```
pub async fn join(
    account: &Account,
    address: impl TryInto<ServerAddress>,
    ecs: &mut Ecs,
) -> Result<Entity, JoinError> {
    let address: ServerAddress = address.try_into().map_err(|_| JoinError::InvalidAddress)?;
    let resolved_address = resolver::resolve_address(&address).await?;

    // An event that causes the schedule to run. This is only used internally.
    let (run_schedule_sender, run_schedule_receiver) = mpsc::unbounded_channel();
    let ecs_lock = azalea_ecs_runner(app, run_schedule_receiver, run_schedule_sender.clone());

    Self::start_client(
        ecs_lock,
        account,
        &address,
        &resolved_address,
        run_schedule_sender,
    )
    .await
}

/// Create a [`Client`] when you already have the ECS made with
/// [`start_ecs`]. You'd usually want to use [`Self::join`] instead.
pub async fn start_client(
    ecs_lock: Arc<Mutex<Ecs>>,
    account: &Account,
    address: &ServerAddress,
    resolved_address: &SocketAddr,
    run_schedule_sender: mpsc::UnboundedSender<()>,
) -> Result<Entity, JoinError> {
    let conn = Connection::new(resolved_address).await?;
    let (conn, game_profile) = Self::handshake(conn, account, address).await?;
    let (read_conn, write_conn) = conn.into_split();

    let mut ecs = ecs_lock.lock();

    // Make the ecs entity for this client
    let entity_mut = ecs.spawn_empty();
    let entity = entity_mut.id();

    // we got the GameConnection, so the server is now connected :)
    let client = Client::new(
        game_profile.clone(),
        entity,
        ecs_lock.clone(),
        run_schedule_sender.clone(),
    );

    let (packet_writer_sender, packet_writer_receiver) = mpsc::unbounded_channel();

    let mut local_player = crate::local_player::LocalPlayer::new(
        entity,
        packet_writer_sender,
        // default to an empty world, it'll be set correctly later when we
        // get the login packet
        Arc::new(RwLock::new(World::default())),
    );

    // start receiving packets
    let packet_receiver = packet_handling::PacketReceiver {
        packets: Arc::new(Mutex::new(Vec::new())),
        run_schedule_sender: run_schedule_sender.clone(),
    };

    let read_packets_task = tokio::spawn(packet_receiver.clone().read_task(read_conn));
    let write_packets_task = tokio::spawn(
        packet_receiver
            .clone()
            .write_task(write_conn, packet_writer_receiver),
    );
    local_player.tasks.push(read_packets_task);
    local_player.tasks.push(write_packets_task);

    let entity = ecs
        .entity_mut(entity)
        .insert(JoinedClientBundle {
            local_player,
            packet_receiver,
            game_profile: GameProfileComponent(game_profile),
            physics_state: PhysicsState::default(),
            local_player_events: LocalPlayerEvents(tx),
            _local: Local,
        })
        .id();

    Ok(entity)
}

/// Do a handshake with the server and get to the game state from the
/// initial handshake state.
///
/// This will also automatically refresh the account's access token if
/// it's expired.
pub async fn handshake(
    mut conn: Connection<ClientboundHandshakePacket, ServerboundHandshakePacket>,
    account: &Account,
    address: &ServerAddress,
) -> Result<
    (
        Connection<ClientboundGamePacket, ServerboundGamePacket>,
        GameProfile,
    ),
    JoinError,
> {
    // handshake
    conn.write(
        ClientIntentionPacket {
            protocol_version: PROTOCOL_VERSION,
            hostname: address.host.clone(),
            port: address.port,
            intention: ConnectionProtocol::Login,
        }
        .get(),
    )
    .await?;
    let mut conn = conn.login();

    // login
    conn.write(
        ServerboundHelloPacket {
            name: account.username.clone(),
            profile_id: None,
        }
        .get(),
    )
    .await?;

    let (conn, profile) = loop {
        let packet = conn.read().await?;
        match packet {
            ClientboundLoginPacket::Hello(p) => {
                debug!("Got encryption request");
                let e = azalea_crypto::encrypt(&p.public_key, &p.nonce).unwrap();

                if let Some(access_token) = &account.access_token {
                    // keep track of the number of times we tried
                    // authenticating so we can give up after too many
                    let mut attempts: usize = 1;

                    while let Err(e) = {
                        let access_token = access_token.lock().clone();
                        conn.authenticate(
                            &access_token,
                            &account
                                .uuid
                                .expect("Uuid must be present if access token is present."),
                            e.secret_key,
                            &p,
                        )
                        .await
                    } {
                        if attempts >= 2 {
                            // if this is the second attempt and we failed
                            // both times, give up
                            return Err(e.into());
                        }
                        if matches!(
                            e,
                            ClientSessionServerError::InvalidSession
                                | ClientSessionServerError::ForbiddenOperation
                        ) {
                            // uh oh, we got an invalid session and have
                            // to reauthenticate now
                            account.refresh().await?;
                        } else {
                            return Err(e.into());
                        }
                        attempts += 1;
                    }
                }

                conn.write(
                    ServerboundKeyPacket {
                        key_bytes: e.encrypted_public_key,
                        encrypted_challenge: e.encrypted_nonce,
                    }
                    .get(),
                )
                .await?;

                conn.set_encryption_key(e.secret_key);
            }
            ClientboundLoginPacket::LoginCompression(p) => {
                debug!("Got compression request {:?}", p.compression_threshold);
                conn.set_compression_threshold(p.compression_threshold);
            }
            ClientboundLoginPacket::GameProfile(p) => {
                debug!("Got profile {:?}", p.game_profile);
                break (conn.game(), p.game_profile);
            }
            ClientboundLoginPacket::LoginDisconnect(p) => {
                debug!("Got disconnect {:?}", p);
                return Err(JoinError::Disconnect { reason: p.reason });
            }
            ClientboundLoginPacket::CustomQuery(p) => {
                debug!("Got custom query {:?}", p);
                conn.write(
                    ServerboundCustomQueryPacket {
                        transaction_id: p.transaction_id,
                        data: None,
                    }
                    .get(),
                )
                .await?;
            }
        }
    };

    Ok((conn, profile))
}

/// A bundle for the components that are present on a local player that received
/// a login packet. If you want to filter for this, just use [`Local`].
#[derive(Bundle)]
pub struct JoinedClientBundle {
    pub local_player: LocalPlayer,
    pub packet_receiver: PacketReceiver,
    pub game_profile: GameProfileComponent,
    pub physics_state: PhysicsState,
    pub local_player_events: LocalPlayerEvents,
    pub _local: Local,
}

pub struct AzaleaPlugin;
impl Plugin for AzaleaPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartWalkEvent>()
            .add_event::<StartSprintEvent>();

        app.add_tick_system_set(
            SystemSet::new()
                .with_system(send_position)
                .with_system(update_in_loaded_chunk)
                .with_system(
                    local_player_ai_step
                        .before("ai_step")
                        .after("sprint_listener"),
                ),
        );

        // fire the Death event when the player dies.
        app.add_system(death_event.after("tick").after("packet"));

        // walk and sprint event listeners
        app.add_system(walk_listener.label("walk_listener").before("travel"))
            .add_system(
                sprint_listener
                    .label("sprint_listener")
                    .before("travel")
                    .before("walk_listener"),
            );

        // add GameProfileComponent when we get an AddPlayerEvent
        app.add_system(
            retroactively_add_game_profile_component
                .after("tick")
                .after("packet"),
        );

        app.add_event::<SendPacketEvent>()
            .add_system(handle_send_packet_event.after("tick").after("packet"));

        app.init_resource::<WorldContainer>();
    }
}

/// This plugin group will add all the default plugins necessary for Azalea to
/// work.
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(RunnerPlugin)
            .add(TickPlugin::default())
            .add(AzaleaPlugin)
            .add(PacketHandlerPlugin)
            .add(EntityPlugin)
            .add(PhysicsPlugin)
            .add(TaskPoolPlugin::default())
            .add(ChatPlugin)
            .add(DisconnectPlugin)
    }
}
