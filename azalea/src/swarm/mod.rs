//! Swarms are a way to conveniently control many bots.

mod chat;
mod events;
pub mod prelude;

use crate::{bot::DefaultBotPlugins, HandleFn};
use azalea_client::{
    chat::ChatPacket,
    client::{azalea_ecs_runner, init_ecs_app, JoinError},
    Account, Client, Event,
};
use azalea_ecs::{
    app::{App, Plugin, PluginGroup, PluginGroupBuilder},
    component::Component,
    ecs::Ecs,
    entity::Entity,
    system::Resource,
};
use azalea_protocol::{
    connect::ConnectionError,
    resolver::{self, ResolverError},
    ServerAddress,
};
use azalea_world::WorldContainer;
use futures::future::join_all;
use log::error;
use parking_lot::{Mutex, RwLock};
use std::{collections::HashMap, future::Future, net::SocketAddr, sync::Arc, time::Duration};
use thiserror::Error;
use tokio::sync::mpsc;

/// A swarm is a way to conveniently control many bots at once, while also
/// being able to control bots at an individual level when desired.
///
/// Swarms are created from [`azalea::swarm::SwarmBuilder`].
///
/// The `S` type parameter is the type of the state for individual bots.
/// It's used to make the [`Swarm::add`] function work.
#[derive(Clone, Resource)]
pub struct Swarm {
    pub ecs_lock: Arc<Mutex<Ecs>>,

    bots: Arc<Mutex<HashMap<Entity, Client>>>,

    // bot_datas: Arc<Mutex<Vec<(Client, S)>>>,
    resolved_address: SocketAddr,
    address: ServerAddress,
    pub world_container: Arc<RwLock<WorldContainer>>,

    bots_tx: mpsc::UnboundedSender<(Option<Event>, Client)>,
    swarm_tx: mpsc::UnboundedSender<SwarmEvent>,

    run_schedule_sender: mpsc::UnboundedSender<()>,
}

/// Create a new [`Swarm`].
pub struct SwarmBuilder<S, SS, Fut, SwarmFut>
where
    S: Default + Send + Sync + Clone + 'static,
    SS: Default + Send + Sync + Clone + 'static,
    Fut: Future<Output = Result<(), anyhow::Error>>,
    SwarmFut: Future<Output = Result<(), anyhow::Error>>,
{
    app: App,
    /// The accounts that are going to join the server.
    accounts: Vec<Account>,
    /// The individual bot states. This must be the same length as `accounts`,
    /// since each bot gets one state.
    states: Vec<S>,
    /// The state for the overall swarm.
    swarm_state: SS,
    /// The function that's called every time a bot receives an [`Event`].
    handler: Option<HandleFn<Fut, S>>,
    /// The function that's called every time the swarm receives a
    /// [`SwarmEvent`].
    swarm_handler: Option<SwarmHandleFn<SwarmFut, SS>>,

    /// How long we should wait between each bot joining the server. Set to
    /// None to have every bot connect at the same time. None is different than
    /// a duration of 0, since if a duration is present the bots will wait for
    /// the previous one to be ready.
    join_delay: Option<std::time::Duration>,
}
impl<S, SS, Fut, SwarmFut> SwarmBuilder<S, SS, Fut, SwarmFut>
where
    Fut: Future<Output = Result<(), anyhow::Error>> + Send + 'static,
    SwarmFut: Future<Output = Result<(), anyhow::Error>> + Send + 'static,
    S: Default + Send + Sync + Clone + Component + 'static,
    SS: Default + Send + Sync + Clone + Resource + 'static,
{
    /// Start creating the swarm.
    #[must_use]
    pub fn new() -> Self {
        Self {
            // we create the app here so plugins can add onto it.
            // the schedules won't run until [`Self::start`] is called.
            app: init_ecs_app(),

            accounts: Vec::new(),
            states: Vec::new(),
            swarm_state: SS::default(),
            handler: None,
            swarm_handler: None,
            join_delay: None,
        }
        .add_plugins(DefaultSwarmPlugins)
        .add_plugins(DefaultBotPlugins)
    }

    /// Add a vec of [`Account`]s to the swarm.
    ///
    /// Use [`Self::add_account`] to only add one account. If you want the
    /// clients to have different default states, add them one at a time with
    /// [`Self::add_account_with_state`].
    #[must_use]
    pub fn add_accounts(mut self, accounts: Vec<Account>) -> Self {
        for account in accounts {
            self = self.add_account(account);
        }
        self
    }
    /// Add a single new [`Account`] to the swarm. Use [`Self::add_accounts`] to
    /// add multiple accounts at a time.
    ///
    /// This will make the state for this client be the default, use
    /// [`Self::add_account_with_state`] to avoid that.
    #[must_use]
    pub fn add_account(self, account: Account) -> Self {
        self.add_account_with_state(account, S::default())
    }
    /// Add an account with a custom initial state. Use just
    /// [`Self::add_account`] to use the Default implementation for the state.
    #[must_use]
    pub fn add_account_with_state(mut self, account: Account, state: S) -> Self {
        self.accounts.push(account);
        self.states.push(state);
        self
    }

    /// Set the function that's called every time a bot receives an [`Event`].
    /// This is the way to handle normal per-bot events.
    ///
    /// You must have exactly one client handler and one swarm handler, calling
    /// this again will replace the old client handler function.
    ///
    /// ```
    /// # use azalea::{prelude::*, swarm::prelude::*};
    /// # let swarm_builder = SwarmBuilder::new().set_swarm_handler(swarm_handle);
    /// swarm_builder.set_handler(handle);
    ///
    /// #[derive(Component, Default, Clone)]
    /// struct State {}
    /// async fn handle(mut bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    ///     Ok(())
    /// }
    ///
    /// # #[derive(Resource, Default, Clone)]
    /// # struct SwarmState {}
    /// # async fn swarm_handle(
    /// #     mut swarm: Swarm,
    /// #     event: SwarmEvent,
    /// #     state: SwarmState,
    /// # ) -> anyhow::Result<()> {
    /// #     Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn set_handler(mut self, handler: HandleFn<Fut, S>) -> Self {
        self.handler = Some(handler);
        self
    }
    /// Set the function that's called every time the swarm receives a
    /// [`SwarmEvent`]. This is the way to handle global swarm events.
    ///
    /// You must have exactly one client handler and one swarm handler, calling
    /// this again will replace the old swarm handler function.
    ///
    /// ```
    /// # use azalea::{prelude::*, swarm::prelude::*};
    /// # let swarm_builder = SwarmBuilder::new().set_handler(handle);
    /// swarm_builder.set_swarm_handler(swarm_handle);
    ///
    /// # #[derive(Component, Default, Clone)]
    /// # struct State {}
    ///
    /// # async fn handle(mut bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    /// #     Ok(())
    /// # }
    ///
    /// #[derive(Resource, Default, Clone)]
    /// struct SwarmState {}
    /// async fn swarm_handle(
    ///     mut swarm: Swarm,
    ///     event: SwarmEvent,
    ///     state: SwarmState,
    /// ) -> anyhow::Result<()> {
    ///     Ok(())
    /// }
    /// ```
    #[must_use]
    pub fn set_swarm_handler(mut self, handler: SwarmHandleFn<SwarmFut, SS>) -> Self {
        self.swarm_handler = Some(handler);
        self
    }

    /// Add a plugin to the swarm.
    #[must_use]
    pub fn add_plugin<T: Plugin>(mut self, plugin: T) -> Self {
        self.app.add_plugin(plugin);
        self
    }
    /// Add a group of plugins to the swarm.
    #[must_use]
    pub fn add_plugins<T: PluginGroup>(mut self, plugin_group: T) -> Self {
        self.app.add_plugins(plugin_group);
        self
    }

    /// Set how long we should wait between each bot joining the server.
    ///
    /// By default, every bot will connect at the same time. If you set this
    /// field, however, the bots will wait for the previous one to have
    /// connected and *then* they'll wait the given duration.
    #[must_use]
    pub fn join_delay(mut self, delay: std::time::Duration) -> Self {
        self.join_delay = Some(delay);
        self
    }

    /// Build this `SwarmBuilder` into an actual [`Swarm`] and join the given
    /// server.
    ///
    /// The `address` argument can be a `&str`, [`ServerAddress`], or anything
    /// that implements `TryInto<ServerAddress>`.
    ///
    /// [`ServerAddress`]: azalea_protocol::ServerAddress
    pub async fn start(self, address: impl TryInto<ServerAddress>) -> Result<(), SwarmStartError> {
        assert_eq!(
            self.accounts.len(),
            self.states.len(),
            "There must be exactly one state per bot."
        );

        // convert the TryInto<ServerAddress> into a ServerAddress
        let address: ServerAddress = match address.try_into() {
            Ok(address) => address,
            Err(_) => return Err(SwarmStartError::InvalidAddress),
        };

        // resolve the address
        let resolved_address = resolver::resolve_address(&address).await?;

        let world_container = Arc::new(RwLock::new(WorldContainer::default()));

        // we can't modify the swarm plugins after this
        let (bots_tx, mut bots_rx) = mpsc::unbounded_channel();
        let (swarm_tx, mut swarm_rx) = mpsc::unbounded_channel();

        let (run_schedule_sender, run_schedule_receiver) = mpsc::unbounded_channel();
        let ecs_lock =
            azalea_ecs_runner(self.app, run_schedule_receiver, run_schedule_sender.clone());

        let swarm = Swarm {
            ecs_lock: ecs_lock.clone(),
            bots: Arc::new(Mutex::new(HashMap::new())),

            resolved_address,
            address,
            world_container,

            bots_tx,

            swarm_tx: swarm_tx.clone(),

            run_schedule_sender,
        };
        ecs_lock.lock().insert_resource(swarm.clone());

        // SwarmBuilder (self) isn't Send so we have to take all the things we need out
        // of it
        let mut swarm_clone = swarm.clone();
        let join_delay = self.join_delay;
        let accounts = self.accounts.clone();
        let states = self.states.clone();

        let join_task = tokio::spawn(async move {
            if let Some(join_delay) = join_delay {
                // if there's a join delay, then join one by one
                for (account, state) in accounts.iter().zip(states) {
                    swarm_clone
                        .add_with_exponential_backoff(account, state.clone())
                        .await;
                    tokio::time::sleep(join_delay).await;
                }
            } else {
                // otherwise, join all at once
                let swarm_borrow = &swarm_clone;
                join_all(accounts.iter().zip(states).map(
                    async move |(account, state)| -> Result<(), JoinError> {
                        swarm_borrow
                            .clone()
                            .add_with_exponential_backoff(account, state.clone())
                            .await;
                        Ok(())
                    },
                ))
                .await;
            }
        });

        let swarm_state = self.swarm_state;

        // Watch swarm_rx and send those events to the swarm_handle.
        let swarm_clone = swarm.clone();
        tokio::spawn(async move {
            while let Some(event) = swarm_rx.recv().await {
                if let Some(swarm_handler) = self.swarm_handler {
                    tokio::spawn((swarm_handler)(
                        swarm_clone.clone(),
                        event,
                        swarm_state.clone(),
                    ));
                }
            }
        });

        // bot events
        while let Some((Some(event), bot)) = bots_rx.recv().await {
            if let Some(handler) = self.handler {
                let state = bot.component::<S>();
                tokio::spawn((handler)(bot, event, state));
            }
        }

        join_task.abort();

        Ok(())
    }
}

impl<S, SS, Fut, SwarmFut> Default for SwarmBuilder<S, SS, Fut, SwarmFut>
where
    Fut: Future<Output = Result<(), anyhow::Error>> + Send + 'static,
    SwarmFut: Future<Output = Result<(), anyhow::Error>> + Send + 'static,
    S: Default + Send + Sync + Clone + Component + 'static,
    SS: Default + Send + Sync + Clone + Resource + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

/// An event about something that doesn't have to do with a single bot.
#[derive(Clone, Debug)]
pub enum SwarmEvent {
    /// All the bots in the swarm have successfully joined the server.
    Login,
    /// The swarm was created. This is only fired once, and it's guaranteed to
    /// be the first event to fire.
    Init,
    /// A bot got disconnected from the server.
    ///
    /// You can implement an auto-reconnect by calling [`Swarm::add`]
    /// with the account from this event.
    Disconnect(Account),
    /// At least one bot received a chat message.
    Chat(ChatPacket),
}

pub type SwarmHandleFn<Fut, SS> = fn(Swarm, SwarmEvent, SS) -> Fut;

#[derive(Error, Debug)]
pub enum SwarmStartError {
    #[error("Invalid address")]
    InvalidAddress,
    #[error(transparent)]
    ResolveAddress(#[from] ResolverError),
    #[error("Join error: {0}")]
    Join(#[from] JoinError),
}

/// Make a bot [`Swarm`].
///
/// [`Swarm`]: struct.Swarm.html
///
/// # Examples
/// ```rust,no_run
/// use azalea::{prelude::*, swarm::prelude::*};
/// use std::time::Duration;
///
/// #[derive(Default, Clone, Component)]
/// struct State {}
///
/// #[derive(Default, Clone, Resource)]
/// struct SwarmState {}
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let mut accounts = Vec::new();
///     let mut states = Vec::new();
///
///     for i in 0..10 {
///         accounts.push(Account::offline(&format!("bot{i}")));
///         states.push(State::default());
///     }
///
///     loop {
///         let e = SwarmBuilder::new()
///             .add_accounts(accounts.clone())
///             .set_handler(handle)
///             .set_swarm_handler(swarm_handle)
///             .join_delay(Duration::from_millis(1000))
///             .start("localhost")
///             .await;
///         println!("{e:?}");
///     }
/// }
///
/// async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
///     match &event {
///         _ => {}
///     }
///     Ok(())
/// }
///
/// async fn swarm_handle(
///     mut swarm: Swarm,
///     event: SwarmEvent,
///     _state: SwarmState,
/// ) -> anyhow::Result<()> {
///     match &event {
///         SwarmEvent::Disconnect(account) => {
///             // automatically reconnect after 5 seconds
///             tokio::time::sleep(Duration::from_secs(5)).await;
///             swarm.add(account, State::default()).await?;
///         }
///         SwarmEvent::Chat(m) => {
///             println!("{}", m.message().to_ansi());
///         }
///         _ => {}
///     }
///     Ok(())
/// }

impl Swarm {
    /// Add a new account to the swarm. You can remove it later by calling
    /// [`Client::disconnect`].
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the bot could not do a handshake successfully.
    pub async fn add<S: Component + Clone>(
        &mut self,
        account: &Account,
        state: S,
    ) -> Result<Client, JoinError> {
        // tx is moved to the bot so it can send us events
        // rx is used to receive events from the bot
        // An event that causes the schedule to run. This is only used internally.
        // let (run_schedule_sender, run_schedule_receiver) = mpsc::unbounded_channel();
        // let ecs_lock = start_ecs(run_schedule_receiver, run_schedule_sender.clone());
        let (bot, mut rx) = Client::start_client(
            self.ecs_lock.clone(),
            account,
            &self.address,
            &self.resolved_address,
            self.run_schedule_sender.clone(),
        )
        .await?;
        // add the state to the client
        {
            let mut ecs = self.ecs_lock.lock();
            ecs.entity_mut(bot.entity).insert(state);
        }

        self.bots.lock().insert(bot.entity, bot.clone());

        let cloned_bots = self.bots.clone();
        let cloned_bots_tx = self.bots_tx.clone();
        let cloned_bot = bot.clone();
        let owned_account = account.clone();
        let swarm_tx = self.swarm_tx.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                // we can't handle events here (since we can't copy the handler),
                // they're handled above in SwarmBuilder::start
                if let Err(e) = cloned_bots_tx.send((Some(event), cloned_bot.clone())) {
                    error!("Error sending event to swarm: {e}");
                }
            }
            cloned_bots.lock().remove(&bot.entity);
            swarm_tx
                .send(SwarmEvent::Disconnect(owned_account))
                .unwrap();
        });

        Ok(bot)
    }

    /// Add a new account to the swarm, retrying if it couldn't join. This will
    /// run forever until the bot joins or the task is aborted.
    ///
    /// Exponential backoff means if it fails joining it will initially wait 10
    /// seconds, then 20, then 40, up to 2 minutes.
    pub async fn add_with_exponential_backoff<S: Component + Clone>(
        &mut self,
        account: &Account,
        state: S,
    ) -> Client {
        let mut disconnects = 0;
        loop {
            match self.add(account, state.clone()).await {
                Ok(bot) => return bot,
                Err(e) => {
                    disconnects += 1;
                    let delay = (Duration::from_secs(5) * 2u32.pow(disconnects))
                        .min(Duration::from_secs(120));
                    let username = account.username.clone();
                    error!("Error joining as {username}: {e}. Waiting {delay:?} and trying again.");
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }
}

impl IntoIterator for Swarm {
    type Item = Client;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    /// Iterate over the bots in this swarm.
    ///
    /// ```rust,no_run
    /// # use azalea::{prelude::*, swarm::prelude::*};
    /// #[derive(Component, Clone)]
    /// # pub struct State;
    /// # fn example(swarm: Swarm) {
    /// for bot in swarm {
    ///     let state = bot.component::<State>();
    ///     // ...
    /// }
    /// # }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.bots
            .lock()
            .clone()
            .into_values()
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl From<ConnectionError> for SwarmStartError {
    fn from(e: ConnectionError) -> Self {
        SwarmStartError::from(JoinError::from(e))
    }
}

/// This plugin group will add all the default plugins necessary for swarms to
/// work.
pub struct DefaultSwarmPlugins;

impl PluginGroup for DefaultSwarmPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(chat::SwarmChatPlugin)
            .add(events::SwarmPlugin)
    }
}
