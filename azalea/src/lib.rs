#![doc = include_str!("../README.md")]
#![allow(incomplete_features)]
#![feature(type_changing_struct_update)]
#![feature(lazy_cell)]
#![feature(let_chains)]
#![feature(never_type)]

pub mod accept_resource_packs;
pub mod auto_respawn;
pub mod auto_tool;
mod bot;
pub mod container;
pub mod nearest_entity;
pub mod pathfinder;
pub mod prelude;
pub mod swarm;

use std::sync::Arc;

use app::Plugins;
use auth::account::Account;
pub use azalea_auth as auth;
pub use azalea_block as blocks;
pub use azalea_brigadier as brigadier;
pub use azalea_buf as buf;
pub use azalea_chat::FormattedText;
pub use azalea_client::*;
pub use azalea_core as core;
// these are re-exported on this level because they're very common
pub use azalea_core::{
    position::{BlockPos, Vec3},
    resource_location::ResourceLocation,
};
pub use azalea_entity as entity;
pub use azalea_physics as physics;
pub use azalea_protocol as protocol;
pub use azalea_registry as registry;
pub use azalea_world as world;
pub use bot::*;
use ecs::component::Component;
use futures::{future::BoxFuture, Future};
use protocol::{resolver::ResolverError, ServerAddress};
use swarm::SwarmBuilder;
use thiserror::Error;

pub use bevy_app as app;
pub use bevy_ecs as ecs;

pub type BoxHandleFn<S> =
    Box<dyn Fn(Client, azalea_client::Event, S) -> BoxFuture<'static, Result<(), anyhow::Error>>>;
pub type HandleFn<S, Fut> = fn(Client, azalea_client::Event, S) -> Fut;

#[derive(Error, Debug)]
pub enum StartError {
    #[error("Invalid address")]
    InvalidAddress,
    #[error(transparent)]
    ResolveAddress(#[from] ResolverError),
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
pub struct ClientBuilder<S>
where
    S: Default + Send + Sync + Clone + Component + 'static,
{
    /// Internally, ClientBuilder is just a wrapper over SwarmBuilder since it's
    /// technically just a subset of it so we can avoid duplicating code this
    /// way.
    swarm: SwarmBuilder<S, swarm::NoSwarmState>,
}
impl ClientBuilder<NoState> {
    /// Start building a client that can join the world.
    #[must_use]
    pub fn new() -> ClientBuilder<NoState> {
        Self::new_without_plugins()
            .add_plugins(DefaultPlugins)
            .add_plugins(DefaultBotPlugins)
    }

    /// [`Self::new`] but without adding the plugins by default. This is useful
    /// if you want to disable a default plugin.
    ///
    /// Note that you can also disable `LogPlugin` by disabling the `log`
    /// feature.
    ///
    /// You **must** add [`DefaultPlugins`] and [`DefaultBotPlugins`] to this.
    ///
    /// ```
    /// # use azalea::prelude::*;
    /// use azalea::{app::PluginGroup, DefaultBotPlugins, DefaultPlugins};
    /// use bevy_log::LogPlugin;
    ///
    /// let client_builder = ClientBuilder::new_without_plugins()
    ///     .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
    ///     .add_plugins(DefaultBotPlugins);
    /// # client_builder.set_handler(handle);
    /// # #[derive(Component, Clone, Default)]
    /// # pub struct State;
    /// # async fn handle(mut bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    /// #     Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn new_without_plugins() -> ClientBuilder<NoState> {
        Self {
            swarm: SwarmBuilder::new_without_plugins(),
        }
    }

    /// Set the function that's called every time a bot receives an [`Event`].
    /// This is the way to handle normal per-bot events.
    ///
    /// Currently you can have up to one client handler.
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
    pub fn set_handler<S, Fut>(self, handler: HandleFn<S, Fut>) -> ClientBuilder<S>
    where
        S: Default + Send + Sync + Clone + Component + 'static,
        Fut: Future<Output = Result<(), anyhow::Error>> + Send + 'static,
    {
        ClientBuilder {
            swarm: self.swarm.set_handler(handler),
        }
    }
}
impl<S> ClientBuilder<S>
where
    S: Default + Send + Sync + Clone + Component + 'static,
{
    /// Set the client state instead of initializing defaults.
    #[must_use]
    pub fn set_state(mut self, state: S) -> Self {
        self.swarm.states = vec![state];
        self
    }
    /// Add a group of plugins to the client.
    #[must_use]
    pub fn add_plugins<M>(mut self, plugins: impl Plugins<M>) -> Self {
        self.swarm = self.swarm.add_plugins(plugins);
        self
    }

    /// Build this `ClientBuilder` into an actual [`Client`] and join the given
    /// server. If the client can't join, it'll keep retrying forever until it
    /// can.
    ///
    /// The `address` argument can be a `&str`, [`ServerAddress`], or anything
    /// that implements `TryInto<ServerAddress>`.
    ///
    /// # Errors
    ///
    /// This will error if the given address is invalid or couldn't be resolved
    /// to a Minecraft server.
    ///
    /// [`ServerAddress`]: azalea_protocol::ServerAddress
    pub async fn start(
        mut self,
        account: impl Account,
        address: impl TryInto<ServerAddress>,
    ) -> Result<!, StartError> {
        self.swarm.accounts = vec![BoxedAccount(Arc::new(account))];
        if self.swarm.states.is_empty() {
            self.swarm.states = vec![S::default()];
        }
        self.swarm.start(address).await
    }
}
impl Default for ClientBuilder<NoState> {
    fn default() -> Self {
        Self::new()
    }
}

/// A marker that can be used in place of a State in [`ClientBuilder`] or
/// [`SwarmBuilder`]. You probably don't need to use this manually since the
/// compiler will infer it for you.
///
/// [`SwarmBuilder`]: swarm::SwarmBuilder
#[derive(Component, Clone, Default)]
pub struct NoState;
