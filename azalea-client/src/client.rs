use std::{
    io,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use azalea_auth::{game_profile::GameProfile, sessionserver::ClientSessionServerError};
use azalea_chat::FormattedText;
use azalea_core::tick::GameTick;
use azalea_entity::{
    indexing::{EntityIdIndex, EntityUuidIndex},
    EntityPlugin, EntityUpdateSet, LocalEntity,
};
use azalea_physics::PhysicsPlugin;
use azalea_protocol::{
    connect::{Connection, ConnectionError, Proxy},
    packets::{
        configuration::{
            serverbound_client_information_packet::ClientInformation,
            ClientboundConfigurationPacket, ServerboundConfigurationPacket,
        },
        handshaking::{
            client_intention_packet::ClientIntentionPacket, ClientboundHandshakePacket,
            ServerboundHandshakePacket,
        },
        login::{
            self, serverbound_custom_query_answer_packet::ServerboundCustomQueryAnswerPacket,
            serverbound_hello_packet::ServerboundHelloPacket,
            serverbound_key_packet::ServerboundKeyPacket,
            serverbound_login_acknowledged_packet::ServerboundLoginAcknowledgedPacket,
            ClientboundLoginPacket,
        },
        ClientIntention, ConnectionProtocol, PROTOCOL_VERSION,
    },
    resolver, ServerAddress,
};

use azalea_world::{Instance, InstanceContainer};
use bevy_app::{
    App, FixedMain, FixedMainScheduleOrder, FixedUpdate, Main, MainSchedulePlugin, Plugin,
    PluginGroup, PluginGroupBuilder, Plugins, PluginsState, PreUpdate, Update,
};
use bevy_ecs::{
    component::Component,
    schedule::IntoSystemConfigs,
    system::{IntoSystem, ResMut, Resource},
    world::World,
};
use bevy_time::{Fixed, Time, TimePlugin, Virtual};
use parking_lot::RwLock;
use tokio::runtime;
use tracing::{debug, error, Level};

use crate::{
    attack::{self, AttackPlugin},
    chat::ChatPlugin,
    chunks::{ChunkBatchInfo, ChunkPlugin},
    configuration::ConfigurationPlugin,
    disconnect::DisconnectPlugin,
    interact::{CurrentSequenceNumber, InteractPlugin},
    inventory::{InventoryComponent, InventoryPlugin},
    local_player::{Hunger, PermissionLevel, PlayerAbilities},
    mining::{self, MinePlugin},
    movement::{LastSentLookDirection, PlayerMovePlugin},
    packet_handling::PacketHandlerPlugin,
    player::retroactively_add_game_profile_component,
    raw_connection::RawConnection,
    respawn::RespawnPlugin,
    task_pool::TaskPoolPlugin,
    Account, GameProfileComponent, InstanceHolder, PhysicsState, TabList,
};

#[derive(Resource)]
pub struct TokioRuntime {
    pub rt: runtime::Runtime,
}

pub struct ClientBuilder<'a> {
    pub app: App,
    pub rt: runtime::Runtime,
    pub account: &'a Account,
    pub address: &'a ServerAddress,
    pub resolved_address: &'a SocketAddr,
    pub proxy: Option<Proxy>,
}

impl<'a> ClientBuilder<'a> {
    pub fn new(
        account: &'a Account,
        address: &'a ServerAddress,
        resolved_address: &'a SocketAddr,
    ) -> ClientBuilder<'a> {
        let rt = runtime::Runtime::new().unwrap();

        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        Self {
            app,
            rt,
            account,
            address,
            resolved_address,
            proxy: None,
        }
    }

    /// Add a group of plugins to the client.
    #[must_use]
    pub fn add_plugins<M>(mut self, plugins: impl Plugins<M>) -> Self {
        self.app.add_plugins(plugins);
        self
    }

    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}

/// A marker component for local players that are currently in the
/// `configuration` state.
#[derive(Component)]
pub struct InConfigurationState;

pub struct AzaleaPlugin;
impl Plugin for AzaleaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                // add GameProfileComponent when we get an AddPlayerEvent
                retroactively_add_game_profile_component.after(EntityUpdateSet::Index),
            ),
        )
        .init_resource::<InstanceContainer>()
        .init_resource::<TabList>();
    }
}

/// An error that happened while joining the server.
#[derive(thiserror::Error, Debug)]
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

impl ClientBuilder<'_> {
    pub fn run(self) -> Result<(), JoinError> {
        self.rt.handle().clone().block_on(self.init())?.run();
        Ok(())
    }

    pub async fn init(mut self) -> Result<App, JoinError> {
        // check if an entity with our uuid already exists in the ecs and if so then
        // just use that
        let entity = {
            let entity_uuid_index = self.app.world.resource::<EntityUuidIndex>();
            let uuid = self.account.uuid_or_offline();
            let entity =
                if let Some(entity) = entity_uuid_index.get(&self.account.uuid_or_offline()) {
                    debug!("Reusing entity {entity:?} for client");
                    entity
                } else {
                    let entity = self.app.world.spawn_empty().id();
                    debug!("Created new entity {entity:?} for client");
                    // add to the uuid index
                    let mut entity_uuid_index = self.app.world.resource_mut::<EntityUuidIndex>();
                    entity_uuid_index.insert(uuid, entity);
                    entity
                };

            // add the Account to the entity now so plugins can access it earlier
            self.app
                .world
                .entity_mut(entity)
                .insert(self.account.to_owned());

            entity
        };

        let (conn, game_profile) = Self::handshake(
            self.account,
            self.address,
            self.resolved_address,
            self.proxy,
        )
        .await?;

        // note that we send the proper packets in
        // crate::configuration::handle_in_configuration_state

        let (read_conn, write_conn) = conn.into_split();
        let (read_conn, write_conn) = (read_conn.raw, write_conn.raw);

        // we did the handshake, so now we're connected to the server

        let instance = Instance::default();
        let instance_holder = crate::local_player::InstanceHolder::new(
            entity,
            // default to an empty world, it'll be set correctly later when we
            // get the login packet
            Arc::new(RwLock::new(instance)),
        );

        self.app.world.entity_mut(entity).insert((
            // these stay when we switch to the game state
            LocalPlayerBundle {
                raw_connection: RawConnection::new(
                    self.rt.handle().clone(),
                    ConnectionProtocol::Configuration,
                    read_conn,
                    write_conn,
                ),
                game_profile: GameProfileComponent(game_profile),
                client_information: crate::ClientInformation::default(),
                instance_holder,
            },
            InConfigurationState,
        ));

        self.app.world.insert_resource(TokioRuntime { rt: self.rt });

        Ok(self.app)
    }

    /// Do a handshake with the server and get to the game state from the
    /// initial handshake state.
    ///
    /// This will also automatically refresh the account's access token if
    /// it's expired.
    pub async fn handshake(
        account: &Account,
        address: &ServerAddress,
        resolved_address: &SocketAddr,
        proxy: Option<Proxy>,
    ) -> Result<
        (
            Connection<ClientboundConfigurationPacket, ServerboundConfigurationPacket>,
            GameProfile,
        ),
        JoinError,
    > {
        let mut conn = if let Some(proxy) = proxy {
            Connection::new_with_proxy(resolved_address, proxy).await?
        } else {
            Connection::new(resolved_address).await?
        };
        // handshake
        conn.write(
            ClientIntentionPacket {
                protocol_version: PROTOCOL_VERSION,
                hostname: address.host.clone(),
                port: address.port,
                intention: ClientIntention::Login,
            }
            .get(),
        )
        .await?;
        let mut conn = conn.login();

        // login
        conn.write(
            ServerboundHelloPacket {
                name: account.username.clone(),
                // TODO: pretty sure this should generate an offline-mode uuid instead of just
                // Uuid::default()
                profile_id: account.uuid.unwrap_or_default(),
            }
            .get(),
        )
        .await?;

        let (conn, profile) = loop {
            let packet = conn.read().await?;

            match packet {
                ClientboundLoginPacket::Hello(p) => {
                    debug!("Got encryption request");
                    let e = azalea_crypto::encrypt(&p.public_key, &p.challenge).unwrap();

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
                            encrypted_challenge: e.encrypted_challenge,
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
                    debug!(
                        "Got profile {:?}. handshake is finished and we're now switching to the configuration state",
                        p.game_profile
                    );
                    conn.write(ServerboundLoginAcknowledgedPacket {}.get())
                        .await?;
                    break (conn.configuration(), p.game_profile);
                }
                ClientboundLoginPacket::LoginDisconnect(p) => {
                    debug!("Got disconnect {:?}", p);
                    return Err(JoinError::Disconnect { reason: p.reason });
                }
                ClientboundLoginPacket::CustomQuery(p) => {
                    debug!("Got custom query {:?}", p);

                    conn.write(
                        ServerboundCustomQueryAnswerPacket {
                            transaction_id: p.transaction_id,
                            data: None,
                        }
                        .get(),
                    )
                    .await?;
                }
                ClientboundLoginPacket::CookieRequest(p) => {
                    debug!("Got cookie request {:?}", p);
                }
            }
        };
        Ok((conn, profile))
    }
}

/// The bundle of components that's shared when we're either in the
/// `configuration` or `game` state.
///
/// For the components that are only present in the `game` state, see
/// [`JoinedClientBundle`] and for the ones in the `configuration` state, see
/// [`ConfigurationClientBundle`].
#[derive(bevy_ecs::bundle::Bundle)]
pub struct LocalPlayerBundle {
    pub raw_connection: RawConnection,
    pub game_profile: GameProfileComponent,
    pub client_information: ClientInformation,
    pub instance_holder: InstanceHolder,
}

/// A bundle for the components that are present on a local player that is
/// currently in the `game` protocol state. If you want to filter for this, just
/// use [`LocalEntity`].
#[derive(bevy_ecs::bundle::Bundle)]
pub struct JoinedClientBundle {
    // note that InstanceHolder isn't here because it's set slightly before we fully join the world
    pub physics_state: PhysicsState,
    pub inventory: InventoryComponent,
    pub tab_list: TabList,
    pub current_sequence_number: CurrentSequenceNumber,
    pub last_sent_direction: LastSentLookDirection,
    pub abilities: PlayerAbilities,
    pub permission_level: PermissionLevel,
    pub chunk_batch_info: ChunkBatchInfo,
    pub hunger: Hunger,

    pub entity_id_index: EntityIdIndex,

    pub mining: mining::MineBundle,
    pub attack: attack::AttackBundle,

    pub _local_entity: LocalEntity,
}

pub struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(GameTick)
            .add_plugins(TimePlugin)
            .insert_resource(Time::<Fixed>::from_seconds(0.05))
            .set_runner(run_loop);

        app.world
            .get_resource_mut::<FixedMainScheduleOrder>()
            .unwrap()
            .insert_after(FixedUpdate, GameTick);
    }
}

fn run_loop(mut app: App) {
    while app.plugins_state() == PluginsState::Adding {
        #[cfg(not(target_arch = "wasm32"))]
        bevy_tasks::tick_global_task_pools_on_main_thread();
    }
    app.finish();
    app.cleanup();

    loop {
        app.update();
    }
}

/// This plugin group will add all the default plugins necessary for Azalea to
/// work.
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        #[allow(unused_mut)]
        let mut group = PluginGroupBuilder::start::<Self>()
            .add(PacketHandlerPlugin)
            .add(AzaleaPlugin)
            .add(EntityPlugin)
            .add(PhysicsPlugin)
            .add(TaskPoolPlugin::default())
            .add(InventoryPlugin)
            .add(ChatPlugin)
            .add(DisconnectPlugin)
            .add(PlayerMovePlugin)
            .add(InteractPlugin)
            .add(RespawnPlugin)
            .add(MinePlugin)
            .add(AttackPlugin)
            .add(ChunkPlugin)
            .add(ConfigurationPlugin)
            .add(TickPlugin);
        #[cfg(feature = "log")]
        {
            let mut log = bevy_log::LogPlugin::default();
            log.level = Level::DEBUG;
            group = group.add(log);
        }
        group
    }
}
