use crate::{
    attack::{self, AttackPlugin},
    chat::ChatPlugin,
    chunks::{ChunkBatchInfo, ChunkPlugin},
    configuration::ConfigurationPlugin,
    disconnect::{DisconnectEvent, DisconnectPlugin},
    events::{Event, EventPlugin, LocalPlayerEvents},
    interact::{CurrentSequenceNumber, InteractPlugin},
    inventory::{InventoryComponent, InventoryPlugin},
    local_player::{
        death_event, GameProfileComponent, Hunger, InstanceHolder, PermissionLevel,
        PlayerAbilities, TabList,
    },
    mining::{self, MinePlugin},
    movement::{LastSentLookDirection, PhysicsState, PlayerMovePlugin},
    packet_handling::{
        login::{self, LoginSendPacketQueue},
        PacketHandlerPlugin,
    },
    player::retroactively_add_game_profile_component,
    raw_connection::RawConnection,
    respawn::RespawnPlugin,
    task_pool::TaskPoolPlugin,
    PlayerInfo,
};

use async_trait::async_trait;
use azalea_auth::{
    account::Account, certs::{Certificates, FetchCertificatesError}, game_profile::GameProfile, sessionserver::ClientSessionServerError
};
use azalea_chat::FormattedText;
use azalea_core::{position::Vec3, tick::GameTick};
use azalea_entity::{
    indexing::{EntityIdIndex, EntityUuidIndex},
    metadata::Health,
    EntityPlugin, EntityUpdateSet, EyeHeight, LocalEntity, Position,
};
use azalea_physics::PhysicsPlugin;
use azalea_protocol::{
    connect::{Connection, ConnectionError},
    packets::{
        configuration::{
            serverbound_client_information_packet::ClientInformation,
            ClientboundConfigurationPacket, ServerboundConfigurationPacket,
        },
        game::ServerboundGamePacket,
        handshaking::{
            client_intention_packet::ClientIntentionPacket, ClientboundHandshakePacket,
            ServerboundHandshakePacket,
        },
        login::{
            serverbound_hello_packet::ServerboundHelloPacket,
            serverbound_key_packet::ServerboundKeyPacket,
            serverbound_login_acknowledged_packet::ServerboundLoginAcknowledgedPacket,
            ClientboundLoginPacket,
        },
        ConnectionProtocol, PROTOCOL_VERSION,
    },
    resolver, ServerAddress,
};
use azalea_world::{Instance, InstanceContainer, InstanceName, PartialInstance};
use bevy_app::{App, Plugin, PluginGroup, PluginGroupBuilder, Update};
use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    entity::Entity,
    schedule::{InternedScheduleLabel, IntoSystemConfigs, LogLevel, ScheduleBuildSettings},
    system::{ResMut, Resource},
    world::World,
};
use bevy_time::TimePlugin;
use derive_more::Deref;
use parking_lot::{Mutex, RwLock};
use std::{
    collections::HashMap,
    fmt::Debug,
    io,
    net::SocketAddr,
    ops::Deref,
    sync::Arc,
    time::{Duration, Instant},
};
use thiserror::Error;
use tokio::{
    sync::{broadcast, mpsc},
    time,
};
use tracing::{debug, error};
use uuid::Uuid;

/// `Client` has the things that a user interacting with the library will want.
///
/// To make a new client, use either [`azalea::ClientBuilder`] or
/// [`Client::join`].
///
/// Note that `Client` is inaccessible from systems (i.e. plugins), but you can
/// achieve everything that client can do with events.
///
/// [`azalea::ClientBuilder`]: https://docs.rs/azalea/latest/azalea/struct.ClientBuilder.html
#[derive(Clone)]
pub struct Client {
    /// The [`GameProfile`] for our client. This contains your username, UUID,
    /// and skin data.
    ///
    /// This is immutable; the server cannot change it. To get the username and
    /// skin the server chose for you, get your player from the [`TabList`]
    /// component.
    ///
    /// This as also available from the ECS as [`GameProfileComponent`].
    pub profile: GameProfile,

    /// The entity for this client in the ECS.
    pub entity: Entity,

    /// The entity component system. You probably don't need to access this
    /// directly. Note that if you're using a shared world (i.e. a swarm), this
    /// will contain all entities in all worlds.
    pub ecs: Arc<Mutex<World>>,

    /// Use this to force the client to run the schedule outside of a tick.
    pub run_schedule_sender: mpsc::UnboundedSender<()>,
}

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
    Auth(#[from] azalea_auth::MicrosoftAuthError),
    #[error("Disconnected: {reason}")]
    Disconnect { reason: FormattedText },
}

#[derive(Clone, Debug, Component)]
pub struct BoxedAccount(pub Arc<dyn Account>);

#[async_trait]
impl Account for BoxedAccount {
    async fn join_with_server_id_hash(
        &self,
        uuid: Uuid,
        server_hash: String,
    ) -> Result<(), ClientSessionServerError> {
        self.0.join_with_server_id_hash(uuid, server_hash).await
    }

    async fn fetch_certificates(&self) -> Result<Option<Certificates>, FetchCertificatesError> {
        self.0.fetch_certificates().await
    }

    fn get_username(&self) -> String {
        self.0.get_username()
    }

    fn get_uuid(&self) -> Uuid {
        self.0.get_uuid()
    }
}


impl Client {
    /// Create a new client from the given [`GameProfile`], ECS Entity, ECS
    /// World, and schedule runner function.
    /// You should only use this if you want to change these fields from the
    /// defaults, otherwise use [`Client::join`].
    pub fn new(
        profile: GameProfile,
        entity: Entity,
        ecs: Arc<Mutex<World>>,
        run_schedule_sender: mpsc::UnboundedSender<()>,
    ) -> Self {
        Self {
            profile,
            // default our id to 0, it'll be set later
            entity,

            ecs,

            run_schedule_sender,
        }
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
        account: impl Account,
        address: impl TryInto<ServerAddress>,
    ) -> Result<(Self, mpsc::UnboundedReceiver<Event>), JoinError> {
        let address: ServerAddress = address.try_into().map_err(|_| JoinError::InvalidAddress)?;
        let resolved_address = resolver::resolve_address(&address).await?;

        // An event that causes the schedule to run. This is only used internally.
        let (run_schedule_sender, run_schedule_receiver) = mpsc::unbounded_channel();

        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        let ecs_lock = start_ecs_runner(app, run_schedule_receiver, run_schedule_sender.clone());

        Self::start_client(
            ecs_lock,
            BoxedAccount(Arc::new(account)),
            &address,
            &resolved_address,
            run_schedule_sender,
        )
        .await
    }

    /// Create a [`Client`] when you already have the ECS made with
    /// [`start_ecs_runner`]. You'd usually want to use [`Self::join`] instead.
    pub async fn start_client(
        ecs_lock: Arc<Mutex<World>>,
        account: BoxedAccount,
        address: &ServerAddress,
        resolved_address: &SocketAddr,
        run_schedule_sender: mpsc::UnboundedSender<()>,
    ) -> Result<(Self, mpsc::UnboundedReceiver<Event>), JoinError> {
        // check if an entity with our uuid already exists in the ecs and if so then
        // just use that
        let entity = {
            let mut ecs = ecs_lock.lock();

            let entity_uuid_index = ecs.resource::<EntityUuidIndex>();
            let uuid = account.0.get_uuid();
            let entity = if let Some(entity) = entity_uuid_index.get(&account.0.get_uuid()) {
                debug!("Reusing entity {entity:?} for client");
                entity
            } else {
                let entity = ecs.spawn_empty().id();
                debug!("Created new entity {entity:?} for client");
                // add to the uuid index
                let mut entity_uuid_index = ecs.resource_mut::<EntityUuidIndex>();
                entity_uuid_index.insert(uuid, entity);
                entity
            };

            // add the Account to the entity now so plugins can access it earlier
            ecs.entity_mut(entity).insert(account.clone());

            entity
        };

        let conn = Connection::new(resolved_address).await?;
        let (conn, game_profile) =
            Self::handshake(ecs_lock.clone(), entity, conn, account, address).await?;

        // note that we send the proper packets in
        // crate::configuration::handle_in_configuration_state

        let (read_conn, write_conn) = conn.into_split();
        let (read_conn, write_conn) = (read_conn.raw, write_conn.raw);

        // we did the handshake, so now we're connected to the server

        let (tx, rx) = mpsc::unbounded_channel();

        let mut ecs = ecs_lock.lock();

        // we got the ConfigurationConnection, so the client is now connected :)
        let client = Client::new(
            game_profile.clone(),
            entity,
            ecs_lock.clone(),
            run_schedule_sender.clone(),
        );

        ecs.entity_mut(entity).insert((
            // these stay when we switch to the game state
            LocalPlayerBundle {
                raw_connection: RawConnection::new(
                    run_schedule_sender,
                    ConnectionProtocol::Configuration,
                    read_conn,
                    write_conn,
                ),
                local_player_events: LocalPlayerEvents(tx),
                game_profile: GameProfileComponent(game_profile),
                client_information: crate::ClientInformation::default(),
            },
            InConfigurationState,
        ));

        Ok((client, rx))
    }

    /// Do a handshake with the server and get to the game state from the
    /// initial handshake state.
    ///
    /// This will also automatically refresh the account's access token if
    /// it's expired.
    pub async fn handshake(
        ecs_lock: Arc<Mutex<World>>,
        entity: Entity,
        mut conn: Connection<ClientboundHandshakePacket, ServerboundHandshakePacket>,
        account: BoxedAccount,
        address: &ServerAddress,
    ) -> Result<
        (
            Connection<ClientboundConfigurationPacket, ServerboundConfigurationPacket>,
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

        // this makes it so plugins can send an `SendLoginPacketEvent` event to the ecs
        // and we'll send it to the server
        let (ecs_packets_tx, mut ecs_packets_rx) = mpsc::unbounded_channel();
        ecs_lock.lock().entity_mut(entity).insert((
            LoginSendPacketQueue { tx: ecs_packets_tx },
            login::IgnoreQueryIds::default(),
        ));

        // login
        conn.write(
            ServerboundHelloPacket {
                name: account.0.get_username().clone(),
                profile_id: account.0.get_uuid(),
            }
            .get(),
        )
        .await?;

        let (conn, profile) = loop {
            let packet = tokio::select! {
                packet = conn.read() => packet?,
                Some(packet) = ecs_packets_rx.recv() => {
                    // write this packet to the server
                    conn.write(packet).await?;
                    continue;
                }
            };

            ecs_lock.lock().send_event(login::LoginPacketEvent {
                entity,
                packet: Arc::new(packet.clone()),
            });

            match packet {
                ClientboundLoginPacket::Hello(p) => {
                    debug!("Got encryption request");
                    let e = azalea_crypto::encrypt(&p.public_key, &p.nonce).unwrap();

                    // keep track of the number of times we tried
                    // authenticating so we can give up after too many
                    let mut attempts: usize = 1;

                    while let Err(e) =
                        { conn.authenticate(account.0.clone(), e.secret_key, &p).await }
                    {
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
                            // account.refresh().await?; FIXME: Make this
                            // work with microsoft accounts or smth
                        } else {
                            return Err(e.into());
                        }
                        attempts += 1;
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
                    // replying to custom query is done in
                    // packet_handling::login::process_packet_events
                }
            }
        };

        ecs_lock
            .lock()
            .entity_mut(entity)
            .remove::<login::IgnoreQueryIds>()
            .remove::<LoginSendPacketQueue>();

        Ok((conn, profile))
    }

    /// Write a packet directly to the server.
    pub fn write_packet(
        &self,
        packet: ServerboundGamePacket,
    ) -> Result<(), crate::raw_connection::WritePacketError> {
        self.raw_connection_mut(&mut self.ecs.lock())
            .write_packet(packet)
    }

    /// Disconnect this client from the server by ending all tasks.
    ///
    /// The OwnedReadHalf for the TCP connection is in one of the tasks, so it
    /// automatically closes the connection when that's dropped.
    pub fn disconnect(&self) {
        self.ecs.lock().send_event(DisconnectEvent {
            entity: self.entity,
            reason: None,
        });
    }

    pub fn raw_connection<'a>(&'a self, ecs: &'a mut World) -> &'a RawConnection {
        self.query::<&RawConnection>(ecs)
    }
    pub fn raw_connection_mut<'a>(
        &'a self,
        ecs: &'a mut World,
    ) -> bevy_ecs::world::Mut<'a, RawConnection> {
        self.query::<&mut RawConnection>(ecs)
    }

    /// Get a component from this client. This will clone the component and
    /// return it.
    ///
    /// # Panics
    ///
    /// This will panic if the component doesn't exist on the client.
    ///
    /// # Examples
    ///
    /// ```
    /// # use azalea_world::InstanceName;
    /// # fn example(client: &azalea_client::Client) {
    /// let world_name = client.component::<InstanceName>();
    /// # }
    pub fn component<T: Component + Clone>(&self) -> T {
        self.query::<&T>(&mut self.ecs.lock()).clone()
    }

    /// Get a component from this client, or `None` if it doesn't exist.
    pub fn get_component<T: Component + Clone>(&self) -> Option<T> {
        self.query::<Option<&T>>(&mut self.ecs.lock()).cloned()
    }

    /// Get an `RwLock` with a reference to our (potentially shared) world.
    ///
    /// This gets the [`Instance`] from the client's [`InstanceHolder`]
    /// component. If it's a normal client, then it'll be the same as the
    /// world the client has loaded. If the client is using a shared world,
    /// then the shared world will be a superset of the client's world.
    pub fn world(&self) -> Arc<RwLock<Instance>> {
        let instance_holder = self.component::<InstanceHolder>();
        instance_holder.instance.clone()
    }

    /// Get an `RwLock` with a reference to the world that this client has
    /// loaded.
    ///
    /// ```
    /// # use azalea_core::position::ChunkPos;
    /// # fn example(client: &azalea_client::Client) {
    /// let world = client.partial_world();
    /// let is_0_0_loaded = world.read().chunks.limited_get(&ChunkPos::new(0, 0)).is_some();
    /// # }
    pub fn partial_world(&self) -> Arc<RwLock<PartialInstance>> {
        let instance_holder = self.component::<InstanceHolder>();
        instance_holder.partial_instance.clone()
    }

    /// Returns whether we have a received the login packet yet.
    pub fn logged_in(&self) -> bool {
        // the login packet tells us the world name
        self.query::<Option<&InstanceName>>(&mut self.ecs.lock())
            .is_some()
    }

    /// Tell the server we changed our game options (i.e. render distance, main
    /// hand). If this is not set before the login packet, the default will
    /// be sent.
    ///
    /// ```rust,no_run
    /// # use azalea_client::{Client, ClientInformation};
    /// # async fn example(bot: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// bot.set_client_information(ClientInformation {
    ///     view_distance: 2,
    ///     ..Default::default()
    /// })
    /// .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_client_information(
        &self,
        client_information: ClientInformation,
    ) -> Result<(), crate::raw_connection::WritePacketError> {
        {
            let mut ecs = self.ecs.lock();
            let mut client_information_mut = self.query::<&mut ClientInformation>(&mut ecs);
            *client_information_mut = client_information.clone();
        }

        if self.logged_in() {
            tracing::debug!(
                "Sending client information (already logged in): {:?}",
                client_information
            );
            self.write_packet(azalea_protocol::packets::game::serverbound_client_information_packet::ServerboundClientInformationPacket { information: client_information.clone() }.get())?;
        }

        Ok(())
    }
}

impl Client {
    /// Get the position of this client.
    ///
    /// This is a shortcut for `Vec3::from(&bot.component::<Position>())`.
    pub fn position(&self) -> Vec3 {
        Vec3::from(&self.component::<Position>())
    }

    /// Get the position of this client's eyes.
    ///
    /// This is a shortcut for
    /// `bot.position().up(bot.component::<EyeHeight>())`.
    pub fn eye_position(&self) -> Vec3 {
        self.position().up((*self.component::<EyeHeight>()) as f64)
    }

    /// Get the health of this client.
    ///
    /// This is a shortcut for `*bot.component::<Health>()`.
    pub fn health(&self) -> f32 {
        *self.component::<Health>()
    }

    /// Get the hunger level of this client, which includes both food and
    /// saturation.
    ///
    /// This is a shortcut for `self.component::<Hunger>().to_owned()`.
    pub fn hunger(&self) -> Hunger {
        self.component::<Hunger>().to_owned()
    }

    /// Get the username of this client.
    ///
    /// This is a shortcut for
    /// `bot.component::<GameProfileComponent>().name.to_owned()`.
    pub fn username(&self) -> String {
        self.component::<GameProfileComponent>().name.to_owned()
    }

    /// Get the Minecraft UUID of this client.
    ///
    /// This is a shortcut for `bot.component::<GameProfileComponent>().uuid`.
    pub fn uuid(&self) -> Uuid {
        self.component::<GameProfileComponent>().uuid
    }

    /// Get a map of player UUIDs to their information in the tab list.
    ///
    /// This is a shortcut for `*bot.component::<TabList>()`.
    pub fn tab_list(&self) -> HashMap<Uuid, PlayerInfo> {
        self.component::<TabList>().deref().clone()
    }
}

/// The bundle of components that's shared when we're either in the
/// `configuration` or `game` state.
///
/// For the components that are only present in the `game` state, see
/// [`JoinedClientBundle`] and for the ones in the `configuration` state, see
/// [`ConfigurationClientBundle`].
#[derive(Bundle)]
pub struct LocalPlayerBundle {
    pub raw_connection: RawConnection,
    pub local_player_events: LocalPlayerEvents,
    pub game_profile: GameProfileComponent,
    pub client_information: ClientInformation,
}

/// A bundle for the components that are present on a local player that is
/// currently in the `game` protocol state. If you want to filter for this, just
/// use [`LocalEntity`].
#[derive(Bundle)]
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
                // fire the Death event when the player dies.
                death_event,
                // add GameProfileComponent when we get an AddPlayerEvent
                retroactively_add_game_profile_component.after(EntityUpdateSet::Index),
            ),
        )
        .init_resource::<InstanceContainer>()
        .init_resource::<TabList>();
    }
}

/// Start running the ECS loop!
///
/// You can create your app with `App::new()`, but don't forget to add
/// [`DefaultPlugins`].
#[doc(hidden)]
pub fn start_ecs_runner(
    app: App,
    run_schedule_receiver: mpsc::UnboundedReceiver<()>,
    run_schedule_sender: mpsc::UnboundedSender<()>,
) -> Arc<Mutex<World>> {
    // all resources should have been added by now so we can take the ecs from the
    // app
    let ecs = Arc::new(Mutex::new(app.world));

    tokio::spawn(run_schedule_loop(
        ecs.clone(),
        app.main_schedule_label,
        run_schedule_receiver,
    ));
    tokio::spawn(tick_run_schedule_loop(run_schedule_sender));

    ecs
}

async fn run_schedule_loop(
    ecs: Arc<Mutex<World>>,
    outer_schedule_label: InternedScheduleLabel,
    mut run_schedule_receiver: mpsc::UnboundedReceiver<()>,
) {
    let mut last_tick: Option<Instant> = None;
    loop {
        // get rid of any queued events
        while let Ok(()) = run_schedule_receiver.try_recv() {}

        // whenever we get an event from run_schedule_receiver, run the schedule
        run_schedule_receiver.recv().await;

        let mut ecs = ecs.lock();

        // if last tick is None or more than 50ms ago, run the GameTick schedule
        if last_tick
            .map(|last_tick| last_tick.elapsed() > Duration::from_millis(50))
            .unwrap_or(true)
        {
            if let Some(last_tick) = &mut last_tick {
                *last_tick += Duration::from_millis(50);
            } else {
                last_tick = Some(Instant::now());
            }
            ecs.run_schedule(GameTick);
        }

        ecs.run_schedule(outer_schedule_label);

        ecs.clear_trackers();
    }
}

/// Send an event to run the schedule every 50 milliseconds. It will stop when
/// the receiver is dropped.
pub async fn tick_run_schedule_loop(run_schedule_sender: mpsc::UnboundedSender<()>) {
    let mut game_tick_interval = time::interval(time::Duration::from_millis(50));
    // TODO: Minecraft bursts up to 10 ticks and then skips, we should too
    game_tick_interval.set_missed_tick_behavior(time::MissedTickBehavior::Burst);

    loop {
        game_tick_interval.tick().await;
        if let Err(e) = run_schedule_sender.send(()) {
            println!("tick_run_schedule_loop error: {e}");
            // the sender is closed so end the task
            return;
        }
    }
}

/// A resource that contains a [`broadcast::Sender`] that will be sent every
/// Minecraft tick.
///
/// This is useful for running code every schedule from async user code.
///
/// ```
/// use azalea_client::TickBroadcast;
/// # async fn example(client: azalea_client::Client) {
/// let mut receiver = {
///     let ecs = client.ecs.lock();
///     let tick_broadcast = ecs.resource::<TickBroadcast>();
///     tick_broadcast.subscribe()
/// };
/// while receiver.recv().await.is_ok() {
///     // do something
/// }
/// # }
/// ```
#[derive(Resource, Deref)]
pub struct TickBroadcast(broadcast::Sender<()>);

pub fn send_tick_broadcast(tick_broadcast: ResMut<TickBroadcast>) {
    let _ = tick_broadcast.0.send(());
}
/// A plugin that makes the [`RanScheduleBroadcast`] resource available.
pub struct TickBroadcastPlugin;
impl Plugin for TickBroadcastPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TickBroadcast(broadcast::channel(1).0))
            .add_systems(GameTick, send_tick_broadcast);
    }
}

pub struct AmbiguityLoggerPlugin;
impl Plugin for AmbiguityLoggerPlugin {
    fn build(&self, app: &mut App) {
        app.edit_schedule(Update, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..Default::default()
            });
        });
        app.edit_schedule(GameTick, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..Default::default()
            });
        });
    }
}

/// This plugin group will add all the default plugins necessary for Azalea to
/// work.
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        #[allow(unused_mut)]
        let mut group = PluginGroupBuilder::start::<Self>()
            .add(AmbiguityLoggerPlugin)
            .add(TimePlugin)
            .add(PacketHandlerPlugin)
            .add(AzaleaPlugin)
            .add(EntityPlugin)
            .add(PhysicsPlugin)
            .add(EventPlugin)
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
            .add(TickBroadcastPlugin);
        #[cfg(feature = "log")]
        {
            group = group.add(bevy_log::LogPlugin::default());
        }
        group
    }
}
