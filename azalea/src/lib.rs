#![doc = include_str!("../README.md")]
#![feature(async_closure)]

mod bot;
pub mod pathfinder;
pub mod prelude;
pub mod swarm;

pub use azalea_block as blocks;
use azalea_client::client::{azalea_ecs_runner, init_ecs_app, JoinError};
pub use azalea_client::*;
pub use azalea_core::{BlockPos, Vec3};
use azalea_ecs::{
    app::{App, Plugin},
    component::Component,
};
pub use azalea_protocol as protocol;
pub use azalea_registry::EntityKind;
pub use azalea_world::{entity, World};
use bot::DefaultBotPlugins;
use ecs::app::PluginGroup;
use futures::Future;
use protocol::{
    resolver::{self, ResolverError},
    ServerAddress,
};
use thiserror::Error;
use tokio::sync::mpsc;

pub type HandleFn<Fut, S> = fn(Client, Event, S) -> Fut;

#[derive(Error, Debug)]
pub enum StartError {
    #[error("Invalid address")]
    InvalidAddress,
    #[error(transparent)]
    ResolveAddress(#[from] ResolverError),
    #[error("Join error: {0}")]
    Join(#[from] JoinError),
}

/// A builder for creating new [`Client`]s. This is the recommended way of
/// making Azalea bots.
///
/// ```no_run
/// # use azalea::prelude::*;
/// # #[tokio::main]
/// # async fn main() {
/// ClientBuilder::new()
///     .set_handler(handle)
///     .start(Account::offline("bot"), "localhost")
///     .await;
/// # }
/// # #[derive(Component, Clone, Default)]
/// # pub struct State;
/// # async fn handle(mut bot: Client, event: Event, state: State) -> anyhow::Result<()> {
/// #     Ok(())
/// # }
/// ```
pub struct ClientBuilder<S, Fut>
where
    S: Default + Send + Sync + Clone + 'static,
    Fut: Future<Output = Result<(), anyhow::Error>>,
{
    app: App,
    /// The function that's called every time a bot receives an [`Event`].
    handler: Option<HandleFn<Fut, S>>,
    state: S,
}
impl<S, Fut> ClientBuilder<S, Fut>
where
    S: Default + Send + Sync + Clone + Component + 'static,
    Fut: Future<Output = Result<(), anyhow::Error>> + Send + 'static,
{
    /// Start building a client that can join the world.
    #[must_use]
    pub fn new() -> Self {
        Self {
            // we create the app here so plugins can add onto it.
            // the schedules won't run until [`Self::start`] is called.
            app: init_ecs_app(),

            handler: None,
            state: S::default(),
        }
        .add_plugins(DefaultBotPlugins)
    }
    /// Set the function that's called every time a bot receives an [`Event`].
    /// This is the way to handle normal per-bot events.
    ///
    /// You must have exactly one client handler, calling this again will
    /// replace the old client handler function.
    ///
    /// ```
    /// # use azalea::prelude::*;
    /// # let client_builder = azalea::ClientBuilder::new();
    /// client_builder.set_handler(handle);
    ///
    /// # #[derive(Component, Clone, Default)]
    /// # pub struct State;
    /// async fn handle(mut bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    ///     Ok(())
    /// }
    /// ```
    #[must_use]
    pub fn set_handler(mut self, handler: HandleFn<Fut, S>) -> Self {
        self.handler = Some(handler);
        self
    }
    /// Add a plugin to the client.
    #[must_use]
    pub fn add_plugin<T: Plugin>(mut self, plugin: T) -> Self {
        self.app.add_plugin(plugin);
        self
    }
    /// Add a group of plugins to the client.
    #[must_use]
    pub fn add_plugins<T: PluginGroup>(mut self, plugin_group: T) -> Self {
        self.app.add_plugins(plugin_group);
        self
    }

    /// Build this `ClientBuilder` into an actual [`Client`] and join the given
    /// server.
    ///
    /// The `address` argument can be a `&str`, [`ServerAddress`], or anything
    /// that implements `TryInto<ServerAddress>`.
    ///
    /// [`ServerAddress`]: azalea_protocol::ServerAddress
    pub async fn start(
        self,
        account: Account,
        address: impl TryInto<ServerAddress>,
    ) -> Result<(), StartError> {
        let address: ServerAddress = address.try_into().map_err(|_| JoinError::InvalidAddress)?;
        let resolved_address = resolver::resolve_address(&address).await?;

        // An event that causes the schedule to run. This is only used internally.
        let (run_schedule_sender, run_schedule_receiver) = mpsc::unbounded_channel();
        let ecs_lock =
            azalea_ecs_runner(self.app, run_schedule_receiver, run_schedule_sender.clone());

        let (bot, mut rx) = Client::start_client(
            ecs_lock,
            &account,
            &address,
            &resolved_address,
            run_schedule_sender,
        )
        .await?;

        while let Some(event) = rx.recv().await {
            if let Some(handler) = self.handler {
                tokio::spawn((handler)(bot.clone(), event.clone(), self.state.clone()));
            }
        }

        Ok(())
    }
}
impl<S, Fut> Default for ClientBuilder<S, Fut>
where
    S: Default + Send + Sync + Clone + Component + 'static,
    Fut: Future<Output = Result<(), anyhow::Error>> + Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}
