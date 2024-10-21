use std::f32::consts::SQRT_2;

use azalea_client::{SprintDirection, WalkDirection};
use azalea_core::{
    direction::CardinalDirection,
    position::{BlockPos, Vec3},
};

use crate::pathfinder::{astar, costs::*};

use super::{default_is_reached, Edge, ExecuteCtx, IsReachedCtx, MoveData, PathfinderCtx};

pub fn basic_move(ctx: &mut PathfinderCtx, node: BlockPos) {
    forward_move(ctx, node);
    ascend_move(ctx, node);
    descend_move(ctx, node);
    diagonal_move(ctx, node);
    descend_forward_1_move(ctx, node);
    downward_move(ctx, node);
}

fn forward_move(ctx: &mut PathfinderCtx, pos: BlockPos) {
    for dir in CardinalDirection::iter() {
        let offset = BlockPos::new(dir.x(), 0, dir.z());

        let mut cost = SPRINT_ONE_BLOCK_COST;

        let break_cost = ctx.world.cost_for_standing(pos + offset, ctx.mining_cache);
        if break_cost == f32::INFINITY {
            continue;
        }
        cost += break_cost;

        ctx.edges.push(Edge {
            movement: astar::Movement {
                target: pos + offset,
                data: MoveData {
                    execute: &execute_forward_move,
                    is_reached: &default_is_reached,
                },
            },
            cost,
        })
    }
}

fn execute_forward_move(mut ctx: ExecuteCtx) {
    let center = ctx.target.center();

    if ctx.mine_while_at_start(ctx.target.up(1)) {
        return;
    }
    if ctx.mine_while_at_start(ctx.target) {
        return;
    }

    ctx.look_at(center);
    ctx.sprint(SprintDirection::Forward);
}

fn ascend_move(ctx: &mut PathfinderCtx, pos: BlockPos) {
    for dir in CardinalDirection::iter() {
        let offset = BlockPos::new(dir.x(), 1, dir.z());

        let break_cost_1 = ctx
            .world
            .cost_for_breaking_block(pos.up(2), ctx.mining_cache);
        if break_cost_1 == f32::INFINITY {
            continue;
        }
        let break_cost_2 = ctx.world.cost_for_standing(pos + offset, ctx.mining_cache);
        if break_cost_2 == f32::INFINITY {
            continue;
        }

        let cost = SPRINT_ONE_BLOCK_COST
            + JUMP_PENALTY
            + *JUMP_ONE_BLOCK_COST
            + break_cost_1
            + break_cost_2;

        ctx.edges.push(Edge {
            movement: astar::Movement {
                target: pos + offset,
                data: MoveData {
                    execute: &execute_ascend_move,
                    is_reached: &ascend_is_reached,
                },
            },
            cost,
        })
    }
}
fn execute_ascend_move(mut ctx: ExecuteCtx) {
    let ExecuteCtx {
        target,
        start,
        position,
        physics,
        ..
    } = ctx;

    if ctx.mine_while_at_start(start.up(2)) {
        return;
    }
    if ctx.mine_while_at_start(target) {
        return;
    }
    if ctx.mine_while_at_start(target.up(1)) {
        return;
    }

    let target_center = target.center();

    ctx.look_at(target_center);
    ctx.walk(WalkDirection::Forward);

    // these checks are to make sure we don't fall if our velocity is too high in
    // the wrong direction

    let x_axis = (start.x - target.x).abs(); // either 0 or 1
    let z_axis = (start.z - target.z).abs(); // either 0 or 1

    let flat_distance_to_next = x_axis as f64 * (target_center.x - position.x)
        + z_axis as f64 * (target_center.z - position.z);
    let side_distance = z_axis as f64 * (target_center.x - position.x).abs()
        + x_axis as f64 * (target_center.z - position.z).abs();

    let lateral_motion = x_axis as f64 * physics.velocity.z + z_axis as f64 * physics.velocity.x;
    if lateral_motion > 0.1 {
        return;
    }

    if flat_distance_to_next > 1.2 || side_distance > 0.2 {
        return;
    }

    if BlockPos::from(position) == start {
        ctx.jump();
    }
}
#[must_use]
pub fn ascend_is_reached(
    IsReachedCtx {
        position, target, ..
    }: IsReachedCtx,
) -> bool {
    BlockPos::from(position) == target || BlockPos::from(position) == target.down(1)
}

fn descend_move(ctx: &mut PathfinderCtx, pos: BlockPos) {
    for dir in CardinalDirection::iter() {
        let dir_delta = BlockPos::new(dir.x(), 0, dir.z());
        let new_horizontal_position = pos + dir_delta;

        let break_cost_1 = ctx
            .world
            .cost_for_passing(new_horizontal_position, ctx.mining_cache);
        if break_cost_1 == f32::INFINITY {
            continue;
        }

        let mut fall_distance = ctx.world.fall_distance(new_horizontal_position);
        if fall_distance > 3 {
            continue;
        }

        if fall_distance == 0 {
            // if the fall distance is 0, set it to 1 so we try mining
            fall_distance = 1
        }

        let new_position = new_horizontal_position.down(fall_distance as i32);

        // only mine if we're descending 1 block
        let break_cost_2;
        if fall_distance == 1 {
            break_cost_2 = ctx.world.cost_for_standing(new_position, ctx.mining_cache);
            if break_cost_2 == f32::INFINITY {
                continue;
            }
        } else {
            // check whether we can stand on the target position
            if !ctx.world.is_standable(new_position) {
                continue;
            }
            break_cost_2 = 0.;
        }

        let cost = WALK_OFF_BLOCK_COST
            + f32::max(
                FALL_N_BLOCKS_COST
                    .get(fall_distance as usize)
                    .copied()
                    // avoid panicking if we fall more than the size of FALL_N_BLOCKS_COST
                    // probably not possible but just in case
                    .unwrap_or(f32::INFINITY),
                CENTER_AFTER_FALL_COST,
            )
            + break_cost_1
            + break_cost_2;

        ctx.edges.push(Edge {
            movement: astar::Movement {
                target: new_position,
                data: MoveData {
                    execute: &execute_descend_move,
                    is_reached: &descend_is_reached,
                },
            },
            cost,
        })
    }
}
fn execute_descend_move(mut ctx: ExecuteCtx) {
    let ExecuteCtx {
        target,
        start,
        position,
        ..
    } = ctx;

    for i in (0..=(start.y - target.y + 1)).rev() {
        if ctx.mine_while_at_start(target.up(i)) {
            return;
        }
    }

    let start_center = start.center();
    let center = target.center();

    let horizontal_distance_from_target = (center - position).horizontal_distance_squared().sqrt();
    let horizontal_distance_from_start = (start.center() - position)
        .horizontal_distance_squared()
        .sqrt();

    let dest_ahead = Vec3::new(
        start_center.x + (center.x - start_center.x) * 1.5,
        center.y,
        start_center.z + (center.z - start_center.z) * 1.5,
    );

    if BlockPos::from(position) != target || horizontal_distance_from_target > 0.25 {
        if horizontal_distance_from_start < 1.25 {
            // this basically just exists to avoid doing spins while we're falling
            ctx.look_at(dest_ahead);
            ctx.walk(WalkDirection::Forward);
        } else {
            ctx.look_at(center);
            ctx.walk(WalkDirection::Forward);
        }
    } else {
        ctx.walk(WalkDirection::None);
    }
}
#[must_use]
pub fn descend_is_reached(
    IsReachedCtx {
        target,
        start,
        position,
        ..
    }: IsReachedCtx,
) -> bool {
    let dest_ahead = BlockPos::new(
        start.x + (target.x - start.x) * 2,
        target.y,
        start.z + (target.z - start.z) * 2,
    );

    (BlockPos::from(position) == target || BlockPos::from(position) == dest_ahead)
        && (position.y - target.y as f64) < 0.5
}

fn descend_forward_1_move(ctx: &mut PathfinderCtx, pos: BlockPos) {
    for dir in CardinalDirection::iter() {
        let dir_delta = BlockPos::new(dir.x(), 0, dir.z());
        let gap_horizontal_position = pos + dir_delta;
        let new_horizontal_position = pos + dir_delta * 2;

        let gap_fall_distance = ctx.world.fall_distance(gap_horizontal_position);
        let fall_distance = ctx.world.fall_distance(new_horizontal_position);

        if fall_distance == 0 || fall_distance > 3 || gap_fall_distance < fall_distance {
            continue;
        }

        let new_position = new_horizontal_position.down(fall_distance as i32);

        // check whether 2 blocks vertically forward are passable
        if !ctx.world.is_passable(new_horizontal_position) {
            continue;
        }
        if !ctx.world.is_passable(gap_horizontal_position) {
            continue;
        }
        // check whether we can stand on the target position
        if !ctx.world.is_standable(new_position) {
            continue;
        }

        let cost = WALK_OFF_BLOCK_COST
            + WALK_ONE_BLOCK_COST
            + f32::max(
                FALL_N_BLOCKS_COST
                    .get(fall_distance as usize)
                    .copied()
                    // avoid panicking if we fall more than the size of FALL_N_BLOCKS_COST
                    // probably not possible but just in case
                    .unwrap_or(f32::INFINITY),
                CENTER_AFTER_FALL_COST,
            );

        ctx.edges.push(Edge {
            movement: astar::Movement {
                target: new_position,
                data: MoveData {
                    execute: &execute_descend_move,
                    is_reached: &descend_is_reached,
                },
            },
            cost,
        })
    }
}

fn diagonal_move(ctx: &mut PathfinderCtx, pos: BlockPos) {
    for dir in CardinalDirection::iter() {
        let right = dir.right();
        let offset = BlockPos::new(dir.x() + right.x(), 0, dir.z() + right.z());
        let left_pos = BlockPos::new(pos.x + dir.x(), pos.y, pos.z + dir.z());
        let right_pos = BlockPos::new(pos.x + right.x(), pos.y, pos.z + right.z());

        // +0.001 so it doesn't unnecessarily go diagonal sometimes
        let mut cost = SPRINT_ONE_BLOCK_COST * SQRT_2 + 0.001;

        let left_passable = ctx.world.is_passable(left_pos);
        let right_passable = ctx.world.is_passable(right_pos);

        if !left_passable && !right_passable {
            continue;
        }

        if !left_passable || !right_passable {
            // add a bit of cost because it'll probably be hugging a wall here
            cost += WALK_ONE_BLOCK_COST / 2.;
        }

        if !ctx.world.is_standable(pos + offset) {
            continue;
        }

        ctx.edges.push(Edge {
            movement: astar::Movement {
                target: pos + offset,
                data: MoveData {
                    execute: &execute_diagonal_move,
                    is_reached: &default_is_reached,
                },
            },
            cost,
        })
    }
}
fn execute_diagonal_move(mut ctx: ExecuteCtx) {
    let target_center = ctx.target.center();

    ctx.look_at(target_center);
    ctx.sprint(SprintDirection::Forward);
}

/// Go directly down, usually by mining.
fn downward_move(ctx: &mut PathfinderCtx, pos: BlockPos) {
    // make sure we land on a solid block after breaking the one below us
    if !ctx.world.is_block_solid(pos.down(2)) {
        return;
    }

    let break_cost = ctx
        .world
        .cost_for_breaking_block(pos.down(1), ctx.mining_cache);
    if break_cost == f32::INFINITY {
        return;
    }

    let cost = FALL_N_BLOCKS_COST[1] + break_cost;

    ctx.edges.push(Edge {
        movement: astar::Movement {
            target: pos.down(1),
            data: MoveData {
                execute: &execute_downward_move,
                is_reached: &default_is_reached,
            },
        },
        cost,
    })
}
fn execute_downward_move(mut ctx: ExecuteCtx) {
    let ExecuteCtx {
        target, position, ..
    } = ctx;

    let target_center = target.center();

    let horizontal_distance_from_target = (target_center - position)
        .horizontal_distance_squared()
        .sqrt();

    if horizontal_distance_from_target > 0.25 {
        ctx.look_at(target_center);
        ctx.walk(WalkDirection::Forward);
    } else if ctx.mine_while_at_start(target) {
        ctx.walk(WalkDirection::None);
    } else if BlockPos::from(position) != target {
        ctx.look_at(target_center);
        ctx.walk(WalkDirection::Forward);
    } else {
        ctx.walk(WalkDirection::None);
    }
}
