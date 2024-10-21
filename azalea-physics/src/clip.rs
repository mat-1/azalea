use azalea_block::BlockState;
use azalea_core::{
    block_hit_result::BlockHitResult,
    direction::Direction,
    math::{self, lerp, EPSILON},
    position::{BlockPos, Vec3},
};
use azalea_inventory::ItemSlot;
use azalea_world::ChunkStorage;
use bevy_ecs::entity::Entity;

use crate::collision::{BlockWithShape, VoxelShape};

#[derive(Debug, Clone)]
pub struct ClipContext {
    pub from: Vec3,
    pub to: Vec3,
    pub block_shape_type: BlockShapeType,
    pub fluid_pick_type: FluidPickType,
    // pub collision_context: EntityCollisionContext,
}
impl ClipContext {
    // minecraft passes in the world and blockpos here... but it doesn't actually
    // seem necessary?
    pub fn block_shape(&self, block_state: BlockState) -> &VoxelShape {
        // TODO: implement the other shape getters
        // (see the ClipContext.Block class in the vanilla source)
        match self.block_shape_type {
            BlockShapeType::Collider => block_state.shape(),
            BlockShapeType::Outline => block_state.shape(),
            BlockShapeType::Visual => block_state.shape(),
            BlockShapeType::FallDamageResetting => block_state.shape(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BlockShapeType {
    Collider,
    Outline,
    Visual,
    FallDamageResetting,
}
#[derive(Debug, Copy, Clone)]
pub enum FluidPickType {
    None,
    SourceOnly,
    Any,
    Water,
}
#[derive(Debug, Clone)]
pub struct EntityCollisionContext {
    pub descending: bool,
    pub entity_bottom: f64,
    pub held_item: ItemSlot,
    // pub can_stand_on_fluid: Box<dyn Fn(&FluidState) -> bool>,
    pub entity: Entity,
}

pub fn clip(chunk_storage: &ChunkStorage, context: ClipContext) -> BlockHitResult {
    traverse_blocks(
        context.from,
        context.to,
        context,
        |ctx, block_pos| {
            let block_state = chunk_storage.get_block_state(block_pos).unwrap_or_default();
            // TODO: add fluid stuff to this (see getFluidState in vanilla source)
            let block_shape = ctx.block_shape(block_state);
            clip_with_interaction_override(&ctx.from, &ctx.to, block_pos, block_shape, &block_state)
            // let block_distance = if let Some(block_hit_result) =
            // block_hit_result {     context.from.distance_squared_to(&
            // block_hit_result.location) } else {
            //     f64::INFINITY
            // };
        },
        |context| {
            let vec = context.from - context.to;
            BlockHitResult::miss(
                context.to,
                Direction::nearest(vec),
                BlockPos::from(context.to),
            )
        },
    )
}

fn clip_with_interaction_override(
    from: &Vec3,
    to: &Vec3,
    block_pos: &BlockPos,
    block_shape: &VoxelShape,
    block_state: &BlockState,
) -> Option<BlockHitResult> {
    let block_hit_result = block_shape.clip(from, to, block_pos);
    println!("block_hit_result: {block_hit_result:?}");
    if let Some(block_hit_result) = block_hit_result {
        // TODO: minecraft calls .getInteractionShape here
        // some blocks (like tall grass) have a physics shape that's different from the
        // interaction shape, so we need to implement BlockState::interaction_shape. lol
        // have fun
        let interaction_shape = block_state.shape();
        let interaction_hit_result = interaction_shape.clip(from, to, block_pos);
        if let Some(interaction_hit_result) = interaction_hit_result {
            if interaction_hit_result.location.distance_squared_to(from)
                < block_hit_result.location.distance_squared_to(from)
            {
                return Some(block_hit_result.with_direction(interaction_hit_result.direction));
            }
        }
        Some(block_hit_result)
    } else {
        block_hit_result
    }
}

pub fn traverse_blocks<C, T>(
    from: Vec3,
    to: Vec3,
    context: C,
    get_hit_result: impl Fn(&C, &BlockPos) -> Option<T>,
    get_miss_result: impl Fn(&C) -> T,
) -> T {
    if from == to {
        return get_miss_result(&context);
    }

    let right_after_end = Vec3 {
        x: lerp(-EPSILON, to.x, from.x),
        y: lerp(-EPSILON, to.y, from.y),
        z: lerp(-EPSILON, to.z, from.z),
    };

    let right_before_start = Vec3 {
        x: lerp(-EPSILON, from.x, to.x),
        y: lerp(-EPSILON, from.y, to.y),
        z: lerp(-EPSILON, from.z, to.z),
    };

    let mut current_block = BlockPos::from(right_before_start);
    if let Some(data) = get_hit_result(&context, &current_block) {
        return data;
    }

    let vec = right_after_end - right_before_start;

    /// Returns either -1, 0, or 1, depending on whether the number is negative,
    /// zero, or positive.
    ///
    /// This function exists because f64::signum doesn't check for 0.
    fn get_number_sign(num: f64) -> f64 {
        if num == 0. {
            0.
        } else {
            num.signum()
        }
    }

    let vec_sign = Vec3 {
        x: get_number_sign(vec.x),
        y: get_number_sign(vec.y),
        z: get_number_sign(vec.z),
    };

    #[rustfmt::skip]
    let percentage_step = Vec3 {
        x: if vec_sign.x == 0. { f64::MAX } else { vec_sign.x / vec.x },
        y: if vec_sign.y == 0. { f64::MAX } else { vec_sign.y / vec.y },
        z: if vec_sign.z == 0. { f64::MAX } else { vec_sign.z / vec.z },
    };

    let mut percentage = Vec3 {
        x: percentage_step.x
            * if vec_sign.x > 0. {
                1. - math::fract(right_before_start.x)
            } else {
                math::fract(right_before_start.x)
            },
        y: percentage_step.y
            * if vec_sign.y > 0. {
                1. - math::fract(right_before_start.y)
            } else {
                math::fract(right_before_start.y)
            },
        z: percentage_step.z
            * if vec_sign.z > 0. {
                1. - math::fract(right_before_start.z)
            } else {
                math::fract(right_before_start.z)
            },
    };

    println!("percentage_step: {percentage_step:?}");
    println!("percentage: {percentage:?}");

    loop {
        if percentage.x > 1. && percentage.y > 1. && percentage.z > 1. {
            return get_miss_result(&context);
        }

        if percentage.x < percentage.y {
            if percentage.x < percentage.z {
                current_block.x += vec_sign.x as i32;
                percentage.x += percentage_step.x;
            } else {
                current_block.z += vec_sign.z as i32;
                percentage.z += percentage_step.z;
            }
        } else if percentage.y < percentage.z {
            current_block.y += vec_sign.y as i32;
            percentage.y += percentage_step.y;
        } else {
            current_block.z += vec_sign.z as i32;
            percentage.z += percentage_step.z;
        }

        if let Some(data) = get_hit_result(&context, &current_block) {
            return data;
        }
    }
}
