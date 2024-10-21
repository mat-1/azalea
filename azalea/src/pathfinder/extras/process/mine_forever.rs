use std::sync::Arc;

use azalea_block::BlockStates;
use azalea_core::position::{BlockPos, Vec3};
use tracing::info;

use crate::{
    auto_tool::StartMiningBlockWithAutoToolEvent,
    ecs::prelude::*,
    pathfinder::{
        self,
        extras::{
            goals::ReachBlockPosGoal,
            utils::{can_reach_block, pick_closest_block},
        },
        GotoEvent,
    },
    LookAtEvent,
};

use super::{Process, ProcessSystemComponents};

#[derive(Clone, Debug)]
pub struct MineForever {
    pub block_states: BlockStates,
}

pub fn mine_forever(
    mine_forever: &MineForever,
    commands: &mut Commands,
    ProcessSystemComponents {
        entity,
        position,
        instance_holder,
        pathfinder,
        mining,
        executing_path,
        mut items_to_pickup_change_acknowledged,
    }: ProcessSystemComponents<'_>,
    items_to_pickup_positions: &[Vec3],
    goto_events: &mut EventWriter<GotoEvent>,
    look_at_events: &mut EventWriter<LookAtEvent>,
    start_mining_block_events: &mut EventWriter<StartMiningBlockWithAutoToolEvent>,
) {
    let mut should_force_recalculate_path = false;

    if !pathfinder.is_calculating {
        if !**items_to_pickup_change_acknowledged {
            should_force_recalculate_path = true;
            **items_to_pickup_change_acknowledged = true;
            println!("items_to_pickup_change_acknowledged = true");
        }
    }

    if !should_force_recalculate_path {
        if mining.is_some() {
            // currently mining, so wait for that to finish
            return;
        }

        if pathfinder.goal.is_some() || executing_path.is_some() {
            // already pathfinding
            return;
        }
    }

    let instance = &instance_holder.instance.read();

    let target_blocks = instance
        .find_blocks(position, &mine_forever.block_states)
        .take(16)
        .collect::<Vec<_>>();

    let chunk_storage = instance.chunks.clone();
    let player_position = BlockPos::from(position);

    let mineable_blocks = target_blocks
        .iter()
        .filter(|target_pos| can_reach_block(&chunk_storage, player_position, **target_pos))
        .copied()
        .collect::<Vec<_>>();

    if !mineable_blocks.is_empty() {
        // pick the closest one and mine it
        let closest_block_pos = pick_closest_block(player_position, &mineable_blocks)
            .expect("there must be a closest block because mineable_blocks wasn't empty");

        look_at_events.send(LookAtEvent {
            entity,
            position: closest_block_pos.center(),
        });
        start_mining_block_events.send(StartMiningBlockWithAutoToolEvent {
            entity,
            position: closest_block_pos,
        });

        println!("start mining block {closest_block_pos:?}");
        return;
    }

    let mut reach_block_goals = Vec::new();
    for target_pos in target_blocks {
        reach_block_goals.push(ReachBlockPosGoal {
            pos: target_pos,
            chunk_storage: chunk_storage.clone(),
        });
    }

    let mut reach_item_goals = Vec::new();
    for &item_position in items_to_pickup_positions {
        println!("item_position: {item_position:?}");
        reach_item_goals.push(pathfinder::goals::RadiusGoal {
            pos: item_position,
            radius: 1.0,
        });
    }

    if reach_block_goals.is_empty() && reach_item_goals.is_empty() {
        info!("MineForever process is done, can't find any more blocks to mine");
        commands.entity(entity).remove::<Process>();
        return;
    }

    goto_events.send(GotoEvent {
        entity,
        goal: Arc::new(pathfinder::goals::OrGoal(
            pathfinder::goals::OrGoals(reach_block_goals),
            pathfinder::goals::ScaleGoal(pathfinder::goals::OrGoals(reach_item_goals), 0.5),
        )),
        successors_fn: pathfinder::moves::default_move,
        allow_mining: true,
    });
}
