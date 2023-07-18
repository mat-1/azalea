#![doc = include_str!("../README.md")]
#![feature(trait_alias)]

pub mod clip;
pub mod collision;

use azalea_block::{Block, BlockState};
use azalea_core::{BlockPos, Vec3};
use azalea_entity::update_bounding_box;
use azalea_entity::{
    clamp_look_direction, metadata::Sprinting, move_relative, Attributes, Jumping, Local,
    LookDirection, Physics, Position,
};
use azalea_world::{Instance, InstanceContainer, InstanceName};
use bevy_app::{App, FixedUpdate, Plugin, Update};
use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    prelude::Event,
    query::With,
    schedule::{IntoSystemConfigs, SystemSet},
    system::{Query, Res},
};
use collision::{move_colliding, MoverType};

/// A Bevy [`SystemSet`] for running physics that makes entities do things.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PhysicsSet;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ForceJumpEvent>()
            .add_systems(
                Update,
                handle_force_jump
                    .before(update_bounding_box)
                    .after(clamp_look_direction),
            )
            .add_systems(FixedUpdate, (ai_step, travel).chain().in_set(PhysicsSet));
    }
}

/// Move the entity with the given acceleration while handling friction,
/// gravity, collisions, and some other stuff.
fn travel(
    mut query: Query<
        (
            &mut Physics,
            &mut LookDirection,
            &mut Position,
            &Attributes,
            &InstanceName,
        ),
        With<Local>,
    >,
    instance_container: Res<InstanceContainer>,
) {
    for (mut physics, direction, mut position, attributes, world_name) in &mut query {
        let world_lock = instance_container
            .get(world_name)
            .expect("All entities should be in a valid world");
        let world = world_lock.read();
        // if !self.is_effective_ai() && !self.is_controlled_by_local_instance() {
        //     // this.calculateEntityAnimation(this, this instanceof FlyingAnimal);
        //     return;
        // }

        let gravity: f64 = 0.08;

        // TODO: slow falling effect
        // let is_falling = self.delta.y <= 0.;

        // TODO: fluids

        // TODO: elytra

        let block_pos_below = get_block_pos_below_that_affects_movement(&position);

        let block_state_below = world
            .chunks
            .get_block_state(&block_pos_below)
            .unwrap_or(BlockState::AIR);
        let block_below: Box<dyn Block> = block_state_below.into();
        let block_friction = block_below.behavior().friction;

        let inertia = if physics.on_ground {
            block_friction * 0.91
        } else {
            0.91
        };

        // this applies the current delta
        let mut movement = handle_relative_friction_and_calculate_movement(
            block_friction,
            &world,
            &mut physics,
            &direction,
            &mut position,
            attributes,
        );

        movement.y -= gravity;

        // if (this.shouldDiscardFriction()) {
        //     this.setDeltaMovement(movement.x, yMovement, movement.z);
        // } else {
        //     this.setDeltaMovement(movement.x * (double)inertia, yMovement *
        // 0.9800000190734863D, movement.z * (double)inertia); }

        // if should_discard_friction(self) {
        if false {
            physics.delta = movement;
        } else {
            physics.delta = Vec3 {
                x: movement.x * inertia as f64,
                y: movement.y * 0.98f64,
                z: movement.z * inertia as f64,
            };
        }
    }
}

/// applies air resistance, calls self.travel(), and some other random
/// stuff.
pub fn ai_step(
    mut query: Query<
        (Entity, &mut Physics, Option<&Jumping>),
        With<Local>,
        // TODO: ai_step should only run for players in loaded chunks
        // With<LocalPlayerInLoadedChunk> maybe there should be an InLoadedChunk/InUnloadedChunk
        // component?
    >,
    mut force_jump_events: EventWriter<ForceJumpEvent>,
) {
    for (entity, mut physics, jumping) in &mut query {
        // vanilla does movement interpolation here, doesn't really matter much for a
        // bot though

        if physics.delta.x.abs() < 0.003 {
            physics.delta.x = 0.;
        }
        if physics.delta.y.abs() < 0.003 {
            physics.delta.y = 0.;
        }
        if physics.delta.z.abs() < 0.003 {
            physics.delta.z = 0.;
        }

        if let Some(jumping) = jumping {
            if **jumping {
                // TODO: jumping in liquids and jump delay

                if physics.on_ground {
                    force_jump_events.send(ForceJumpEvent(entity));
                }
            }
        }

        physics.xxa *= 0.98;
        physics.zza *= 0.98;

        // TODO: freezing, pushEntities, drowning damage (in their own systems,
        // after `travel`)
    }
}

/// Jump even if we aren't on the ground.
#[derive(Event)]
pub struct ForceJumpEvent(pub Entity);

pub fn handle_force_jump(
    mut query: Query<(
        &mut Physics,
        &Position,
        &LookDirection,
        &Sprinting,
        &InstanceName,
    )>,
    instance_container: Res<InstanceContainer>,
    mut events: EventReader<ForceJumpEvent>,
) {
    for event in events.iter() {
        if let Ok((mut physics, position, direction, sprinting, world_name)) =
            query.get_mut(event.0)
        {
            let world_lock = instance_container
                .get(world_name)
                .expect("All entities should be in a valid world");
            let world = world_lock.read();

            let jump_power: f64 = jump_power(&world, position) as f64 + jump_boost_power();
            let old_delta_movement = physics.delta;
            physics.delta = Vec3 {
                x: old_delta_movement.x,
                y: jump_power,
                z: old_delta_movement.z,
            };
            if **sprinting {
                // sprint jumping gives some extra velocity
                let y_rot = direction.y_rot * 0.017453292;
                physics.delta += Vec3 {
                    x: (-f32::sin(y_rot) * 0.2) as f64,
                    y: 0.,
                    z: (f32::cos(y_rot) * 0.2) as f64,
                };
            }

            physics.has_impulse = true;
        }
    }
}

fn get_block_pos_below_that_affects_movement(position: &Position) -> BlockPos {
    BlockPos::new(
        position.x.floor() as i32,
        // TODO: this uses bounding_box.min_y instead of position.y
        (position.y - 0.5f64).floor() as i32,
        position.z.floor() as i32,
    )
}

fn handle_relative_friction_and_calculate_movement(
    block_friction: f32,
    world: &Instance,
    physics: &mut Physics,
    direction: &LookDirection,
    position: &mut Position,
    attributes: &Attributes,
) -> Vec3 {
    move_relative(
        physics,
        direction,
        get_friction_influenced_speed(physics, attributes, block_friction),
        &Vec3 {
            x: physics.xxa as f64,
            y: physics.yya as f64,
            z: physics.zza as f64,
        },
    );
    // entity.delta = entity.handle_on_climbable(entity.delta);
    move_colliding(
        &MoverType::Own,
        &physics.delta.clone(),
        world,
        position,
        physics,
    )
    .expect("Entity should exist.");
    // let delta_movement = entity.delta;
    // ladders
    //   if ((entity.horizontalCollision || entity.jumping) && (entity.onClimbable()
    // || entity.getFeetBlockState().is(Blocks.POWDER_SNOW) &&
    // PowderSnowBlock.canEntityWalkOnPowderSnow(entity))) {      var3 = new
    // Vec3(var3.x, 0.2D, var3.z);   }
    // TODO: powdered snow

    physics.delta
}

// private float getFrictionInfluencedSpeed(float friction) {
//     return this.onGround ? this.getSpeed() * (0.21600002F / (friction *
// friction * friction)) : this.flyingSpeed; }
fn get_friction_influenced_speed(physics: &Physics, attributes: &Attributes, friction: f32) -> f32 {
    // TODO: have speed & flying_speed fields in entity
    if physics.on_ground {
        let speed: f32 = attributes.speed.calculate() as f32;
        speed * (0.216f32 / (friction * friction * friction))
    } else {
        // entity.flying_speed
        0.02
    }
}

/// Returns the what the entity's jump should be multiplied by based on the
/// block they're standing on.
fn block_jump_factor(world: &Instance, position: &Position) -> f32 {
    let block_at_pos = world.chunks.get_block_state(&position.into());
    let block_below = world
        .chunks
        .get_block_state(&get_block_pos_below_that_affects_movement(position));

    let block_at_pos_jump_factor = if let Some(block) = block_at_pos {
        Box::<dyn Block>::from(block).behavior().jump_factor
    } else {
        1.
    };
    if block_at_pos_jump_factor != 1. {
        return block_at_pos_jump_factor;
    }

    if let Some(block) = block_below {
        Box::<dyn Block>::from(block).behavior().jump_factor
    } else {
        1.
    }
}

// protected float getJumpPower() {
//     return 0.42F * this.getBlockJumpFactor();
// }
// public double getJumpBoostPower() {
//     return this.hasEffect(MobEffects.JUMP) ? (double)(0.1F *
// (float)(this.getEffect(MobEffects.JUMP).getAmplifier() + 1)) : 0.0D; }
fn jump_power(world: &Instance, position: &Position) -> f32 {
    0.42 * block_jump_factor(world, position)
}

fn jump_boost_power() -> f64 {
    // TODO: potion effects
    // if let Some(effects) = entity.effects() {
    //     if let Some(jump_effect) = effects.get(&Effect::Jump) {
    //         0.1 * (jump_effect.amplifier + 1) as f32
    //     } else {
    //         0.
    //     }
    // } else {
    //     0.
    // }
    0.
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use azalea_core::{ChunkPos, ResourceLocation};
    use azalea_entity::{EntityBundle, EntityPlugin};
    use azalea_world::{Chunk, MinecraftEntityId, PartialInstance};
    use bevy_app::App;
    use bevy_time::fixed_timestep::FixedTime;
    use uuid::Uuid;

    /// You need an app to spawn entities in the world and do updates.
    fn make_test_app() -> App {
        let mut app = App::new();
        app.add_plugins((PhysicsPlugin, EntityPlugin))
            .insert_resource(FixedTime::new(Duration::from_millis(50)))
            .init_resource::<InstanceContainer>();
        app
    }

    #[test]
    fn test_gravity() {
        let mut app = make_test_app();
        let _world_lock = app.world.resource_mut::<InstanceContainer>().insert(
            ResourceLocation::new("minecraft:overworld"),
            384,
            -64,
        );

        let entity = app
            .world
            .spawn((
                EntityBundle::new(
                    Uuid::nil(),
                    Vec3 {
                        x: 0.,
                        y: 70.,
                        z: 0.,
                    },
                    azalea_registry::EntityKind::Zombie,
                    ResourceLocation::new("minecraft:overworld"),
                ),
                MinecraftEntityId(0),
                Local,
            ))
            .id();
        {
            let entity_pos = *app.world.get::<Position>(entity).unwrap();
            // y should start at 70
            assert_eq!(entity_pos.y, 70.);
        }
        app.world.run_schedule(FixedUpdate);
        app.update();
        {
            let entity_pos = *app.world.get::<Position>(entity).unwrap();
            // delta is applied before gravity, so the first tick only sets the delta
            assert_eq!(entity_pos.y, 70.);
            let entity_physics = app.world.get::<Physics>(entity).unwrap().clone();
            assert!(entity_physics.delta.y < 0.);
        }
        app.world.run_schedule(FixedUpdate);
        app.update();
        {
            let entity_pos = *app.world.get::<Position>(entity).unwrap();
            // the second tick applies the delta to the position, so now it should go down
            assert!(
                entity_pos.y < 70.,
                "Entity y ({}) didn't go down after physics steps",
                entity_pos.y
            );
        }
    }
    #[test]
    fn test_collision() {
        let mut app = make_test_app();
        let world_lock = app.world.resource_mut::<InstanceContainer>().insert(
            ResourceLocation::new("minecraft:overworld"),
            384,
            -64,
        );
        let mut partial_world = PartialInstance::default();

        partial_world.chunks.set(
            &ChunkPos { x: 0, z: 0 },
            Some(Chunk::default()),
            &mut world_lock.write().chunks,
        );
        let entity = app
            .world
            .spawn((
                EntityBundle::new(
                    Uuid::nil(),
                    Vec3 {
                        x: 0.5,
                        y: 70.,
                        z: 0.5,
                    },
                    azalea_registry::EntityKind::Player,
                    ResourceLocation::new("minecraft:overworld"),
                ),
                MinecraftEntityId(0),
                Local,
            ))
            .id();
        let block_state = partial_world.chunks.set_block_state(
            &BlockPos { x: 0, y: 69, z: 0 },
            azalea_registry::Block::Stone.into(),
            &mut world_lock.write().chunks,
        );
        assert!(
            block_state.is_some(),
            "Block state should exist, if this fails that means the chunk wasn't loaded and the block didn't get placed"
        );
        app.world.run_schedule(FixedUpdate);
        app.update();
        {
            let entity_pos = *app.world.get::<Position>(entity).unwrap();
            // delta will change, but it won't move until next tick
            assert_eq!(entity_pos.y, 70.);
            let entity_physics = app.world.get::<Physics>(entity).unwrap().clone();
            assert!(entity_physics.delta.y < 0.);
        }
        app.world.run_schedule(FixedUpdate);
        app.update();
        {
            let entity_pos = *app.world.get::<Position>(entity).unwrap();
            // the second tick applies the delta to the position, but it also does collision
            assert_eq!(entity_pos.y, 70.);
        }
    }

    #[test]
    fn test_slab_collision() {
        let mut app = make_test_app();
        let world_lock = app.world.resource_mut::<InstanceContainer>().insert(
            ResourceLocation::new("minecraft:overworld"),
            384,
            -64,
        );
        let mut partial_world = PartialInstance::default();

        partial_world.chunks.set(
            &ChunkPos { x: 0, z: 0 },
            Some(Chunk::default()),
            &mut world_lock.write().chunks,
        );
        let entity = app
            .world
            .spawn((
                EntityBundle::new(
                    Uuid::nil(),
                    Vec3 {
                        x: 0.5,
                        y: 71.,
                        z: 0.5,
                    },
                    azalea_registry::EntityKind::Player,
                    ResourceLocation::new("minecraft:overworld"),
                ),
                MinecraftEntityId(0),
                Local,
            ))
            .id();
        let block_state = partial_world.chunks.set_block_state(
            &BlockPos { x: 0, y: 69, z: 0 },
            azalea_block::blocks::StoneSlab {
                kind: azalea_block::properties::Type::Bottom,
                waterlogged: false,
            }
            .into(),
            &mut world_lock.write().chunks,
        );
        assert!(
            block_state.is_some(),
            "Block state should exist, if this fails that means the chunk wasn't loaded and the block didn't get placed"
        );
        // do a few steps so we fall on the slab
        for _ in 0..20 {
            app.world.run_schedule(FixedUpdate);
            app.update();
        }
        let entity_pos = app.world.get::<Position>(entity).unwrap();
        assert_eq!(entity_pos.y, 69.5);
    }

    #[test]
    fn test_top_slab_collision() {
        let mut app = make_test_app();
        let world_lock = app.world.resource_mut::<InstanceContainer>().insert(
            ResourceLocation::new("minecraft:overworld"),
            384,
            -64,
        );
        let mut partial_world = PartialInstance::default();

        partial_world.chunks.set(
            &ChunkPos { x: 0, z: 0 },
            Some(Chunk::default()),
            &mut world_lock.write().chunks,
        );
        let entity = app
            .world
            .spawn((
                EntityBundle::new(
                    Uuid::nil(),
                    Vec3 {
                        x: 0.5,
                        y: 71.,
                        z: 0.5,
                    },
                    azalea_registry::EntityKind::Player,
                    ResourceLocation::new("minecraft:overworld"),
                ),
                MinecraftEntityId(0),
                Local,
            ))
            .id();
        let block_state = world_lock.write().chunks.set_block_state(
            &BlockPos { x: 0, y: 69, z: 0 },
            azalea_block::blocks::StoneSlab {
                kind: azalea_block::properties::Type::Top,
                waterlogged: false,
            }
            .into(),
        );
        assert!(
            block_state.is_some(),
            "Block state should exist, if this fails that means the chunk wasn't loaded and the block didn't get placed"
        );
        // do a few steps so we fall on the slab
        for _ in 0..20 {
            app.world.run_schedule(FixedUpdate);
            app.update();
        }
        let entity_pos = app.world.get::<Position>(entity).unwrap();
        assert_eq!(entity_pos.y, 70.);
    }

    #[test]
    fn test_weird_wall_collision() {
        let mut app = make_test_app();
        let world_lock = app.world.resource_mut::<InstanceContainer>().insert(
            ResourceLocation::new("minecraft:overworld"),
            384,
            -64,
        );
        let mut partial_world = PartialInstance::default();

        partial_world.chunks.set(
            &ChunkPos { x: 0, z: 0 },
            Some(Chunk::default()),
            &mut world_lock.write().chunks,
        );
        let entity = app
            .world
            .spawn((
                EntityBundle::new(
                    Uuid::nil(),
                    Vec3 {
                        x: 0.5,
                        y: 73.,
                        z: 0.5,
                    },
                    azalea_registry::EntityKind::Player,
                    ResourceLocation::new("minecraft:overworld"),
                ),
                MinecraftEntityId(0),
                Local,
            ))
            .id();
        let block_state = world_lock.write().chunks.set_block_state(
            &BlockPos { x: 0, y: 69, z: 0 },
            azalea_block::blocks::CobblestoneWall {
                east: azalea_block::properties::EastWall::Low,
                north: azalea_block::properties::NorthWall::Low,
                south: azalea_block::properties::SouthWall::Low,
                west: azalea_block::properties::WestWall::Low,
                up: false,
                waterlogged: false,
            }
            .into(),
        );
        assert!(
            block_state.is_some(),
            "Block state should exist, if this fails that means the chunk wasn't loaded and the block didn't get placed"
        );
        // do a few steps so we fall on the wall
        for _ in 0..20 {
            app.world.run_schedule(FixedUpdate);
            app.update();
        }

        let entity_pos = app.world.get::<Position>(entity).unwrap();
        assert_eq!(entity_pos.y, 70.5);
    }

    #[test]
    fn test_negative_coordinates_weird_wall_collision() {
        let mut app = make_test_app();
        let world_lock = app.world.resource_mut::<InstanceContainer>().insert(
            ResourceLocation::new("minecraft:overworld"),
            384,
            -64,
        );
        let mut partial_world = PartialInstance::default();

        partial_world.chunks.set(
            &ChunkPos { x: -1, z: -1 },
            Some(Chunk::default()),
            &mut world_lock.write().chunks,
        );
        let entity = app
            .world
            .spawn((
                EntityBundle::new(
                    Uuid::nil(),
                    Vec3 {
                        x: -7.5,
                        y: 73.,
                        z: -7.5,
                    },
                    azalea_registry::EntityKind::Player,
                    ResourceLocation::new("minecraft:overworld"),
                ),
                MinecraftEntityId(0),
                Local,
            ))
            .id();
        let block_state = world_lock.write().chunks.set_block_state(
            &BlockPos {
                x: -8,
                y: 69,
                z: -8,
            },
            azalea_block::blocks::CobblestoneWall {
                east: azalea_block::properties::EastWall::Low,
                north: azalea_block::properties::NorthWall::Low,
                south: azalea_block::properties::SouthWall::Low,
                west: azalea_block::properties::WestWall::Low,
                up: false,
                waterlogged: false,
            }
            .into(),
        );
        assert!(
            block_state.is_some(),
            "Block state should exist, if this fails that means the chunk wasn't loaded and the block didn't get placed"
        );
        // do a few steps so we fall on the wall
        for _ in 0..20 {
            app.world.run_schedule(FixedUpdate);
            app.update();
        }

        let entity_pos = app.world.get::<Position>(entity).unwrap();
        assert_eq!(entity_pos.y, 70.5);
    }
}
