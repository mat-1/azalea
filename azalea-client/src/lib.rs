//! Significantly abstract [`azalea_protocol`] so it's actually useable for
//! real clients. If you want to make bots, you should use the
//! [`azalea`] crate instead.
//!
//! [`azalea_protocol`]: https://docs.rs/azalea-protocol
//! [`azalea`]: https://docs.rs/azalea

#![feature(provide_any)]
#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(error_generic_member_access)]
#![feature(type_alias_impl_trait)]

mod account;
pub mod attack;
pub mod chat;
mod client;
pub mod disconnect;
mod entity_query;
mod events;
mod get_mc_dir;
pub mod interact;
pub mod inventory;
mod local_player;
pub mod mining;
pub mod movement;
pub mod packet_handling;
pub mod ping;
mod player;
pub mod respawn;
pub mod task_pool;

pub use account::{Account, AccountOpts};
pub use client::{
    start_ecs, Client, ClientInformation, DefaultPlugins, JoinError, JoinedClientBundle, TabList,
    TickBroadcast,
};
pub use events::Event;
pub use local_player::{GameProfileComponent, LocalPlayer, LocalPlayerInLoadedChunk};
pub use movement::{SprintDirection, StartSprintEvent, StartWalkEvent, WalkDirection};
pub use player::PlayerInfo;
