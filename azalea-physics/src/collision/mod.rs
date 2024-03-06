mod blocks;
mod discrete_voxel_shape;
mod mergers;
mod shape;
mod world_collisions;

use std::ops::Add;

use azalea_core::{aabb::AABB, direction::Axis, math::EPSILON, position::Vec3};
use azalea_entity::{metadata::ShiftKeyDown, PlayerAbilities};
use azalea_world::{Instance, MoveEntityError};
use bevy_ecs::world::Mut;
pub use blocks::BlockWithShape;
pub use discrete_voxel_shape::*;
pub use shape::*;

use self::world_collisions::get_block_collisions;

pub enum MoverType {
    Own,
    Player,
    Piston,
    ShulkerBox,
    Shulker,
}

// private Vec3 collide(Vec3 var1) {
//     AABB var2 = this.getBoundingBox();
//     List var3 = this.level.getEntityCollisions(this,
// var2.expandTowards(var1));     Vec3 var4 = var1.lengthSqr() == 0.0D ?
// var1 : collideBoundingBox(this, var1, var2, this.level, var3);
//     boolean var5 = var1.x != var4.x;
//     boolean var6 = var1.y != var4.y;
//     boolean var7 = var1.z != var4.z;
//     boolean var8 = this.onGround || var6 && var1.y < 0.0D;
//     if (this.maxUpStep > 0.0F && var8 && (var5 || var7)) {
//        Vec3 var9 = collideBoundingBox(this, new Vec3(var1.x,
// (double)this.maxUpStep, var1.z), var2, this.level, var3);        Vec3
// var10 = collideBoundingBox(this, new Vec3(0.0D, (double)this.maxUpStep,
// 0.0D), var2.expandTowards(var1.x, 0.0D, var1.z), this.level, var3);
//        if (var10.y < (double)this.maxUpStep) {
//           Vec3 var11 = collideBoundingBox(this, new Vec3(var1.x, 0.0D,
// var1.z), var2.move(var10), this.level, var3).add(var10);           if
// (var11.horizontalDistanceSqr() > var9.horizontalDistanceSqr()) {
//              var9 = var11;
//           }
//        }

//        if (var9.horizontalDistanceSqr() > var4.horizontalDistanceSqr()) {
//           return var9.add(collideBoundingBox(this, new Vec3(0.0D, -var9.y +
// var1.y, 0.0D), var2.move(var9), this.level, var3));        }
//     }

//     return var4;
// }
fn collide(movement: &Vec3, world: &Instance, physics: &azalea_entity::Physics) -> Vec3 {
    let entity_bounding_box = physics.bounding_box;
    // TODO: get_entity_collisions
    // let entity_collisions = world.get_entity_collisions(self,
    // entity_bounding_box.expand_towards(movement));
    let entity_collisions = Vec::new();
    let collided_delta = if movement.length_sqr() == 0.0 {
        *movement
    } else {
        collide_bounding_box(
            movement,
            &entity_bounding_box,
            world,
            entity_collisions.clone(),
        )
    };

    let x_collision = movement.x != collided_delta.x;
    let y_collision = movement.y != collided_delta.y;
    let z_collision = movement.z != collided_delta.z;

    let on_ground = physics.on_ground || y_collision && movement.y < 0.;

    let max_up_step = 0.6;
    if max_up_step > 0. && on_ground && (x_collision || z_collision) {
        let mut step_to_delta = collide_bounding_box(
            &Vec3 {
                x: movement.x,
                y: max_up_step,
                z: movement.z,
            },
            &entity_bounding_box,
            world,
            entity_collisions.clone(),
        );
        let directly_up_delta = collide_bounding_box(
            &Vec3 {
                x: 0.,
                y: max_up_step,
                z: 0.,
            },
            &entity_bounding_box.expand_towards(&Vec3::new(movement.x, 0., movement.z)),
            world,
            entity_collisions.clone(),
        );
        if directly_up_delta.y < max_up_step {
            let target_movement = collide_bounding_box(
                &Vec3 {
                    x: movement.x,
                    y: 0.,
                    z: movement.z,
                },
                &entity_bounding_box.move_relative(&directly_up_delta),
                world,
                entity_collisions.clone(),
            )
            .add(directly_up_delta);
            if target_movement.horizontal_distance_sqr() > step_to_delta.horizontal_distance_sqr() {
                step_to_delta = target_movement;
            }
        }

        if step_to_delta.horizontal_distance_sqr() > collided_delta.horizontal_distance_sqr() {
            return step_to_delta.add(collide_bounding_box(
                &Vec3 {
                    x: 0.,
                    y: -step_to_delta.y + movement.y,
                    z: 0.,
                },
                &entity_bounding_box.move_relative(&step_to_delta),
                world,
                entity_collisions.clone(),
            ));
        }
    }

    collided_delta
}

/// Move an entity by a given delta, checking for collisions.
pub fn move_colliding(
    _mover_type: &MoverType,
    movement: &Vec3,
    world: &Instance,
    position: &mut Mut<azalea_entity::Position>,
    physics: &mut azalea_entity::Physics,
    abilities: &PlayerAbilities,
) -> Result<(), MoveEntityError> {
    // TODO: do all these

    // if self.no_physics {
    //     return;
    // };

    // if (var1 == MoverType.PISTON) {
    //     var2 = this.limitPistonMovement(var2);
    //     if (var2.equals(Vec3.ZERO)) {
    //        return;
    //     }
    // }

    // if (this.stuckSpeedMultiplier.lengthSqr() > 1.0E-7D) {
    //     var2 = var2.multiply(this.stuckSpeedMultiplier);
    //     this.stuckSpeedMultiplier = Vec3.ZERO;
    //     this.setDeltaMovement(Vec3.ZERO);
    // }

    // movement = this.maybeBackOffFromEdge(movement, moverType);

    let collide_result = collide(movement, world, physics);

    let move_distance = collide_result.length_sqr();

    if move_distance > EPSILON {
        // TODO: fall damage

        let new_pos = {
            Vec3 {
                x: position.x + collide_result.x,
                y: position.y + collide_result.y,
                z: position.z + collide_result.z,
            }
        };

        if new_pos != ***position {
            ***position = new_pos;
        }
    }

    let x_collision = movement.x != collide_result.x;
    let z_collision = movement.z != collide_result.z;
    let horizontal_collision = x_collision || z_collision;
    let vertical_collision = movement.y != collide_result.y;
    let on_ground = vertical_collision && movement.y < 0.;

    physics.horizontal_collision = horizontal_collision;
    physics.vertical_collision = vertical_collision;
    physics.on_ground = on_ground;

    // TODO: minecraft checks for a "minor" horizontal collision here

    let _block_pos_below = azalea_entity::on_pos_legacy(&world.chunks, position);
    // let _block_state_below = self
    //     .world
    //     .get_block_state(&block_pos_below)
    //     .expect("Couldn't get block state below");

    // self.check_fall_damage(collide_result.y, on_ground, block_state_below,
    // block_pos_below);

    // if self.isRemoved() { return; }

    if horizontal_collision {
        let delta_movement = &physics.velocity;
        physics.velocity = Vec3 {
            x: if x_collision { 0. } else { delta_movement.x },
            y: delta_movement.y,
            z: if z_collision { 0. } else { delta_movement.z },
        }
    }

    if vertical_collision {
        // blockBelow.updateEntityAfterFallOn(this.level, this);
        // the default implementation of updateEntityAfterFallOn sets the y movement to
        // 0
        physics.velocity.y = 0.;
    }

    if on_ground {
        // blockBelow.stepOn(this.level, blockPosBelow, blockStateBelow,
        // this);
    }

    // sounds

    // this.tryCheckInsideBlocks();

    // float var25 = this.getBlockSpeedFactor();
    // this.setDeltaMovement(this.getDeltaMovement().multiply((double)var25, 1.0D,
    // (double)var25)); if (this.level.getBlockStatesIfLoaded(this.
    // getBoundingBox().deflate(1.0E-6D)).noneMatch((var0) -> {
    //    return var0.is(BlockTags.FIRE) || var0.is(Blocks.LAVA);
    // })) {
    //    if (this.remainingFireTicks <= 0) {
    //       this.setRemainingFireTicks(-this.getFireImmuneTicks());
    //    }

    //    if (this.wasOnFire && (this.isInPowderSnow ||
    // this.isInWaterRainOrBubble())) {       this.
    // playEntityOnFireExtinguishedSound();    }
    // }

    // if (this.isOnFire() && (this.isInPowderSnow || this.isInWaterRainOrBubble()))
    // {    this.setRemainingFireTicks(-this.getFireImmuneTicks());
    // }

    Ok(())
}

fn maybe_back_off_from_edge(
    abilities: &PlayerAbilities,
    movement: &Vec3,
    mover_type: MoverType,
    is_shift_key_held: &ShiftKeyDown
) -> Vec3 {
    // if (!this.abilities.flying
    //     && movement.y <= 0.0
    //     && (moverType == MoverType.SELF || moverType == MoverType.PLAYER)
    //     && this.isStayingOnGroundSurface()
    //     && this.isAboveGround()) {

    let is_backing_off = !abilities.flying
        && movement.y <= 0.
        && matches!(mover_type, MoverType::Own | MoverType::Player)
        
}

fn collide_bounding_box(
    movement: &Vec3,
    entity_bounding_box: &AABB,
    world: &Instance,
    entity_collisions: Vec<VoxelShape>,
) -> Vec3 {
    let mut collision_boxes: Vec<VoxelShape> = Vec::with_capacity(entity_collisions.len() + 1);

    if !entity_collisions.is_empty() {
        collision_boxes.extend(entity_collisions);
    }

    // TODO: world border

    let block_collisions =
        get_block_collisions(world, entity_bounding_box.expand_towards(movement));
    collision_boxes.extend(block_collisions);
    collide_with_shapes(movement, *entity_bounding_box, &collision_boxes)
}

fn collide_with_shapes(
    movement: &Vec3,
    mut entity_box: AABB,
    collision_boxes: &Vec<VoxelShape>,
) -> Vec3 {
    if collision_boxes.is_empty() {
        return *movement;
    }

    let mut x_movement = movement.x;
    let mut y_movement = movement.y;
    let mut z_movement = movement.z;
    if y_movement != 0. {
        y_movement = Shapes::collide(&Axis::Y, &entity_box, collision_boxes, y_movement);
        if y_movement != 0. {
            entity_box = entity_box.move_relative(&Vec3 {
                x: 0.,
                y: y_movement,
                z: 0.,
            });
        }
    }

    // whether the player is moving more in the z axis than x
    // this is done to fix a movement bug, minecraft does this too
    let more_z_movement = x_movement.abs() < z_movement.abs();

    if more_z_movement && z_movement != 0. {
        z_movement = Shapes::collide(&Axis::Z, &entity_box, collision_boxes, z_movement);
        if z_movement != 0. {
            entity_box = entity_box.move_relative(&Vec3 {
                x: 0.,
                y: 0.,
                z: z_movement,
            });
        }
    }

    if x_movement != 0. {
        x_movement = Shapes::collide(&Axis::X, &entity_box, collision_boxes, x_movement);
        if x_movement != 0. {
            entity_box = entity_box.move_relative(&Vec3 {
                x: x_movement,
                y: 0.,
                z: 0.,
            });
        }
    }

    if !more_z_movement && z_movement != 0. {
        z_movement = Shapes::collide(&Axis::Z, &entity_box, collision_boxes, z_movement);
    }

    Vec3 {
        x: x_movement,
        y: y_movement,
        z: z_movement,
    }
}
