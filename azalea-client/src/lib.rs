#![doc = include_str!("../README.md")]
#![feature(provide_any)]
#![allow(incomplete_features)]
#![feature(trait_upcasting)]
#![feature(error_generic_member_access)]
#![feature(type_alias_impl_trait)]

mod account;
pub mod chat;
pub mod client;
pub mod disconnect;
mod entity_query;
mod get_mc_dir;
mod local_player;
mod movement;
pub mod packet_handling;
pub mod ping;
mod player;
pub mod runner;
pub mod task_pool;

pub use account::Account;
pub use azalea_ecs as ecs;
pub use local_player::{GameProfileComponent, LocalPlayer};
pub use movement::{SprintDirection, StartSprintEvent, StartWalkEvent, WalkDirection};
pub use player::PlayerInfo;
