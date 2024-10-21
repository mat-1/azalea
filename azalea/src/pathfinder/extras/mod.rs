//! Adds utility functions that all depend on the pathfinder.

pub mod goals;
pub mod pickup;
pub mod process;
pub mod utils;

use crate::ecs::prelude::*;
use azalea_block::BlockStates;
use azalea_client::{mining::MiningSet, Client};
use azalea_core::{position::BlockPos, tick::GameTick};
use azalea_physics::PhysicsSet;
use bevy_app::Update;

use crate::app::{App, Plugin};

use self::process::{mine_area::MineArea, Process, SetActiveProcessEvent};

pub struct PathfinderExtrasPlugin;

impl Plugin for PathfinderExtrasPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetActiveProcessEvent>()
            .add_systems(
                Update,
                (
                    process::set_active_pathfinder_process_listener
                        .after(crate::pathfinder::stop_pathfinding_on_instance_change)
                        .before(crate::pathfinder::handle_stop_pathfinding_event),
                    pickup::add_pickup_components_to_player,
                    pickup::remove_pickup_components_from_player,
                    (
                        pickup::watch_for_mined_blocks,
                        pickup::watch_for_item_spawns_from_blocks_we_mined,
                    )
                        .chain()
                        .after(MiningSet),
                ),
            )
            .add_systems(
                GameTick,
                (
                    pickup::remove_despawned_items_to_pickup,
                    process::process_tick.before(PhysicsSet),
                )
                    .chain(),
            );
    }
}

pub trait PathfinderExtrasClientExt {
    fn set_active_pathfinder_process(&self, process: impl Into<Process>);
    fn mine_area(&self, corner1: BlockPos, corner2: BlockPos);
    fn mine_forever(&self, block_states: impl Into<BlockStates>);
}

impl PathfinderExtrasClientExt for Client {
    fn set_active_pathfinder_process(&self, process: impl Into<Process>) {
        let process = process.into();
        self.ecs.lock().send_event(SetActiveProcessEvent {
            entity: self.entity,
            process,
        });
    }

    fn mine_area(&self, corner1: BlockPos, corner2: BlockPos) {
        self.set_active_pathfinder_process(MineArea { corner1, corner2 });
    }

    fn mine_forever(&self, block_states: impl Into<BlockStates>) {
        let block_states = block_states.into();
        self.set_active_pathfinder_process(process::mine_forever::MineForever { block_states });
    }
}
