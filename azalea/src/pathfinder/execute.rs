use std::{sync::Arc, time::Duration};

use azalea_client::{
    movement::PhysicsState, SprintDirection, StartSprintEvent, StartWalkEvent, WalkDirection,
};
use azalea_core::{BlockPos, ResourceLocation, Vec3};
use azalea_entity::{metadata::Sprinting, Attributes, EyeHeight, LookDirection, Physics, Position};
use azalea_world::{ChunkStorage, Instance, InstanceContainer, InstanceName, MinecraftEntityId};
use bevy_app::{App, FixedUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_time::fixed_timestep::FixedTime;
use log::debug;
use parking_lot::RwLock;

use crate::bot::{JumpEvent, LookAtEvent};

use super::Pathfinder;

pub fn tick_execute_path(
    mut query: Query<(
        Entity,
        &mut Pathfinder,
        &Position,
        &EyeHeight,
        &Physics,
        &PhysicsState,
        &Attributes,
        &InstanceName,
    )>,
    mut look_at_events: EventWriter<LookAtEvent>,
    mut sprint_events: EventWriter<StartSprintEvent>,
    mut walk_events: EventWriter<StartWalkEvent>,
    mut jump_events: EventWriter<JumpEvent>,
    instance_container: Res<InstanceContainer>,
) {
    for (
        entity,
        mut pathfinder,
        position,
        eye_height,
        physics,
        physics_state,
        attributes,
        instance_name,
    ) in &mut query
    {
        let instance = instance_container.get(instance_name).unwrap();
        let simulated_bundle = SimulatedPlayerBundle {
            position: position.clone(),
            physics: physics.clone(),
            physics_state: physics_state.clone(),
            attributes: attributes.clone(),
        };

        loop {
            if pathfinder.path.is_empty() {
                break;
            }

            let potential_targets = pathfinder
                .path
                .clone()
                .into_iter()
                .enumerate()
                .skip(2)
                // only skip up to 10 nodes
                .take(10)
                .rev()
                .collect::<Vec<_>>();

            for (i, movement) in &potential_targets {
                if can_walk_to_position(
                    instance.read().chunks.clone(),
                    simulated_bundle.clone(),
                    SimulationSettings {
                        target: movement.target,
                    },
                ) {
                    // we can skip some nodes
                    pathfinder.path = pathfinder.path.split_off(*i);
                    break;
                }
            }

            let movement = pathfinder.path.front().unwrap().clone();

            if Some(movement.target) != pathfinder.current_target_node {
                // check if we should jump for this movement
                if movement.data.jump {
                    jump_events.send(JumpEvent(entity));
                }
                pathfinder.current_target_node = Some(movement.target);
            }

            let target = movement.target;
            look_at_events.send(LookAtEvent {
                entity,
                position: Vec3 {
                    // look forward
                    y: position.y + **eye_height as f64,
                    ..target.center()
                },
            });
            debug!("tick: pathfinder {entity:?}; going to {target:?}; currently at {position:?}");
            if movement.data.sprint {
                sprint_events.send(StartSprintEvent {
                    entity,
                    direction: SprintDirection::Forward,
                });
            } else {
                println!("not sprinting");
                walk_events.send(StartWalkEvent {
                    entity,
                    direction: WalkDirection::Forward,
                });
            }

            if crate::pathfinder::is_reached(&target, position, physics) {
                if let Some(new_path) = pathfinder.queued_path.take() {
                    pathfinder.path = new_path;
                } else {
                    pathfinder.path.pop_front();
                }
                if pathfinder.path.is_empty() {
                    // println!("reached goal");
                    walk_events.send(StartWalkEvent {
                        entity,
                        direction: WalkDirection::None,
                    });
                }
                // tick again, maybe we already reached the next node!
            } else {
                break;
            }
        }
    }
}

fn can_walk_to_position(
    chunks: ChunkStorage,
    player: SimulatedPlayerBundle,
    settings: SimulationSettings,
) -> bool {
    let mut simulation = Simulation::new(chunks, player, settings.clone());
    simulation.app.add_systems(Update, simulation_init);
    simulation.app.add_systems(FixedUpdate, simulation_tick);

    let start_pos = simulation.position();

    // simulate for 1 second then check the results
    for _ in 0..20 {
        simulation.tick();
        let current_pos = simulation.position();
        if current_pos.y != start_pos.y || simulation.horizontal_collision() {
            return false;
        }
        if BlockPos::from(current_pos) == settings.target {
            return true;
        }
    }
    false
}
fn simulation_init(
    mut query: Query<Entity, Added<MinecraftEntityId>>,
    mut start_sprint_events: EventWriter<StartSprintEvent>,
) {
    for entity in query.iter_mut() {
        // look at the target and start sprinting when they're spawned
        start_sprint_events.send(StartSprintEvent {
            entity,
            direction: SprintDirection::Forward,
        });
    }
}
fn simulation_tick(
    mut query: Query<(&Position, &mut LookDirection)>,
    settings: Res<SimulationSettings>,
) {
    for (position, mut look_direction) in query.iter_mut() {
        let (y_rot, x_rot) = crate::bot::direction_looking_at(&position, &settings.target.center());
        (look_direction.y_rot, look_direction.x_rot) = (y_rot, x_rot);
    }
}

#[derive(Resource, Clone)]
pub struct SimulationSettings {
    pub target: BlockPos,
}

#[derive(Bundle, Clone)]
pub struct SimulatedPlayerBundle {
    pub position: Position,
    pub physics: Physics,
    pub physics_state: PhysicsState,
    pub attributes: Attributes,
}

/// Simulate the Minecraft world to see if certain movements would be possible.
pub struct Simulation {
    pub app: App,
    pub entity: Entity,
    _instance: Arc<RwLock<Instance>>,
}

impl Simulation {
    pub fn new(
        chunks: ChunkStorage,
        player: SimulatedPlayerBundle,
        settings: SimulationSettings,
    ) -> Self {
        let instance_name = ResourceLocation::new("azalea:simulation");

        let instance = Arc::new(RwLock::new(Instance {
            chunks,
            ..Default::default()
        }));

        let mut app = App::new();
        app.add_plugins((
            azalea_physics::PhysicsPlugin,
            azalea_entity::EntityPlugin,
            azalea_client::movement::PlayerMovePlugin,
        ))
        // make sure it doesn't do fixed ticks without us telling it to
        .insert_resource(FixedTime::new(Duration::from_secs(60)))
        .insert_resource(InstanceContainer {
            worlds: [(instance_name.clone(), Arc::downgrade(&instance.clone()))]
                .iter()
                .cloned()
                .collect(),
        })
        .insert_resource(settings);

        app.edit_schedule(bevy_app::Main, |schedule| {
            schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
        });

        let entity = app
            .world
            .spawn((
                MinecraftEntityId(0),
                InstanceName(instance_name),
                azalea_entity::Local,
                azalea_client::LocalPlayerInLoadedChunk,
                azalea_entity::Jumping::default(),
                azalea_entity::LookDirection::default(),
                Sprinting(false),
                player,
            ))
            .id();

        Self {
            app,
            entity,
            _instance: instance,
        }
    }
    pub fn tick(&mut self) {
        self.app.world.run_schedule(FixedUpdate);
        self.app.update();
    }
    pub fn position(&self) -> Vec3 {
        *self.app.world.get::<Position>(self.entity).unwrap().clone()
    }
    pub fn horizontal_collision(&self) -> bool {
        self.app
            .world
            .get::<Physics>(self.entity)
            .unwrap()
            .horizontal_collision
    }
}
