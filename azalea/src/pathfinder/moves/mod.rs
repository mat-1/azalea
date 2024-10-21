pub mod basic;
pub mod parkour;

use std::{fmt::Debug, sync::Arc};

use crate::{auto_tool::StartMiningBlockWithAutoToolEvent, JumpEvent, LookAtEvent};

use super::{
    astar,
    mining::MiningCache,
    world::{is_block_state_passable, CachedWorld},
};
use azalea_client::{
    inventory::SetSelectedHotbarSlotEvent, SprintDirection, StartSprintEvent, StartWalkEvent,
    WalkDirection,
};
use azalea_core::position::{BlockPos, Vec3};
use azalea_inventory::Menu;
use azalea_world::Instance;
use bevy_ecs::{entity::Entity, event::EventWriter};
use parking_lot::RwLock;

type Edge = astar::Edge<BlockPos, MoveData>;

pub type SuccessorsFn = fn(&mut PathfinderCtx, BlockPos);

pub fn default_move(ctx: &mut PathfinderCtx, node: BlockPos) {
    basic::basic_move(ctx, node);
    parkour::parkour_move(ctx, node);
}

#[derive(Clone)]
pub struct MoveData {
    /// Use the context to determine what events should be sent to complete this
    /// movement.
    pub execute: &'static (dyn Fn(ExecuteCtx) + Send + Sync),
    /// Whether we've reached the target.
    pub is_reached: &'static (dyn Fn(IsReachedCtx) -> bool + Send + Sync),
}
impl Debug for MoveData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MoveData")
            // .field("move_kind", &self.move_kind)
            .finish()
    }
}

pub struct ExecuteCtx<'w1, 'w2, 'w3, 'w4, 'w5, 'w6, 'a> {
    pub entity: Entity,
    /// The node that we're trying to reach.
    pub target: BlockPos,
    /// The last node that we reached.
    pub start: BlockPos,
    pub position: Vec3,
    pub physics: &'a azalea_entity::Physics,
    pub is_currently_mining: bool,
    pub instance: Arc<RwLock<Instance>>,
    pub menu: Menu,

    pub look_at_events: &'a mut EventWriter<'w1, LookAtEvent>,
    pub sprint_events: &'a mut EventWriter<'w2, StartSprintEvent>,
    pub walk_events: &'a mut EventWriter<'w3, StartWalkEvent>,
    pub jump_events: &'a mut EventWriter<'w4, JumpEvent>,
    pub start_mining_events: &'a mut EventWriter<'w5, StartMiningBlockWithAutoToolEvent>,
    pub set_selected_hotbar_slot_events: &'a mut EventWriter<'w6, SetSelectedHotbarSlotEvent>,
}

impl ExecuteCtx<'_, '_, '_, '_, '_, '_, '_> {
    pub fn look_at(&mut self, position: Vec3) {
        self.look_at_events.send(LookAtEvent {
            entity: self.entity,
            position: Vec3 {
                x: position.x,
                // look forward
                y: self.position.up(1.53).y,
                z: position.z,
            },
        });
    }

    pub fn look_at_exact(&mut self, position: Vec3) {
        self.look_at_events.send(LookAtEvent {
            entity: self.entity,
            position,
        });
    }

    pub fn sprint(&mut self, direction: SprintDirection) {
        self.sprint_events.send(StartSprintEvent {
            entity: self.entity,
            direction,
        });
    }

    pub fn walk(&mut self, direction: WalkDirection) {
        self.walk_events.send(StartWalkEvent {
            entity: self.entity,
            direction,
        });
    }

    pub fn jump(&mut self) {
        self.jump_events.send(JumpEvent {
            entity: self.entity,
        });
    }

    /// Returns whether this block could be mined.
    pub fn should_mine(&mut self, block: BlockPos) -> bool {
        let block_state = self
            .instance
            .read()
            .get_block_state(&block)
            .unwrap_or_default();
        if is_block_state_passable(block_state) {
            // block is already passable, no need to mine it
            return false;
        }

        true
    }

    /// Mine the block at the given position. Returns whether the block is being
    /// mined.
    pub fn mine(&mut self, block: BlockPos) -> bool {
        let block_state = self
            .instance
            .read()
            .get_block_state(&block)
            .unwrap_or_default();
        if is_block_state_passable(block_state) {
            // block is already passable, no need to mine it
            return false;
        }

        self.is_currently_mining = true;

        self.walk(WalkDirection::None);
        self.look_at_exact(block.center());
        self.start_mining_events
            .send(StartMiningBlockWithAutoToolEvent {
                entity: self.entity,
                position: block,
            });

        true
    }

    /// Mine the given block, but make sure the player is standing at the start
    /// of the current node first.
    pub fn mine_while_at_start(&mut self, block: BlockPos) -> bool {
        let horizontal_distance_from_start = (self.start.center() - self.position)
            .horizontal_distance_squared()
            .sqrt();
        let at_start_position =
            BlockPos::from(self.position) == self.start && horizontal_distance_from_start < 0.25;

        if self.should_mine(block) {
            if at_start_position {
                self.mine(block);
            } else {
                self.look_at(self.start.center());
                self.walk(WalkDirection::Forward);
            }
            true
        } else {
            false
        }
    }
}

pub struct IsReachedCtx<'a> {
    /// The node that we're trying to reach.
    pub target: BlockPos,
    /// The last node that we reached.
    pub start: BlockPos,
    pub position: Vec3,
    pub physics: &'a azalea_entity::Physics,
}

/// Returns whether the entity is at the node and should start going to the
/// next node.
#[must_use]
pub fn default_is_reached(
    IsReachedCtx {
        position, target, ..
    }: IsReachedCtx,
) -> bool {
    BlockPos::from(position) == target
}

pub struct PathfinderCtx<'a> {
    pub edges: &'a mut Vec<Edge>,
    pub world: &'a CachedWorld,
    pub mining_cache: &'a MiningCache,
}
