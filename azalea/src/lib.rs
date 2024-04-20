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

use std::net::SocketAddr;

use app::Plugins;
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
use protocol::connect::Proxy;
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
        account: Account,
        address: impl TryInto<ServerAddress>,
    ) -> Result<!, StartError> {
        self.swarm.accounts = vec![(account, JoinOpts::default())];
        if self.swarm.states.is_empty() {
            self.swarm.states = vec![S::default()];
        }
        self.swarm.start(address).await
    }

    /// Do the same as [`Self::start`], but allow passing in custom join
    /// options.
    pub async fn start_with_opts(
        mut self,
        account: Account,
        address: impl TryInto<ServerAddress>,
        opts: JoinOpts,
    ) -> Result<!, StartError> {
        self.swarm.accounts = vec![(account, opts.clone())];
        if self.swarm.states.is_empty() {
            self.swarm.states = vec![S::default()];
        }
        self.swarm.start_with_default_opts(address, opts).await
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

/// Optional settings when adding an account to a swarm or client.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct JoinOpts {
    /// The Socks5 proxy that this bot will use.
    pub proxy: Option<Proxy>,
    /// Override the server address that this specific bot will send in the
    /// handshake packet.
    pub custom_address: Option<ServerAddress>,
    /// Override the socket address that this specific bot will use to connect
    /// to the server.
    pub custom_resolved_address: Option<SocketAddr>,
}

impl JoinOpts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, other: &Self) {
        if let Some(proxy) = other.proxy.clone() {
            self.proxy = Some(proxy);
        }
        if let Some(custom_address) = other.custom_address.clone() {
            self.custom_address = Some(custom_address);
        }
        if let Some(custom_resolved_address) = other.custom_resolved_address {
            self.custom_resolved_address = Some(custom_resolved_address);
        }
    }

    /// Set the proxy that this bot will use.
    #[must_use]
    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
    /// Set the custom address that this bot will send in the handshake packet.
    #[must_use]
    pub fn custom_address(mut self, custom_address: ServerAddress) -> Self {
        self.custom_address = Some(custom_address);
        self
    }
    /// Set the custom resolved address that this bot will use to connect to the
    /// server.
    #[must_use]
    pub fn custom_resolved_address(mut self, custom_resolved_address: SocketAddr) -> Self {
        self.custom_resolved_address = Some(custom_resolved_address);
        self
    }
}
