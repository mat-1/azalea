//! Simulate the Minecraft world, currently only used for tests.

use std::sync::Arc;

use azalea_client::{
    inventory::InventoryComponent, packet_handling::game::SendPacketEvent, PhysicsState,
};
use azalea_core::{position::Vec3, resource_location::ResourceLocation, tick::GameTick};
use azalea_entity::{
    attributes::AttributeInstance, Attributes, EntityDimensions, LookDirection, Physics, Position,
};
use azalea_world::{ChunkStorage, Instance, InstanceContainer, MinecraftEntityId, PartialInstance};
use bevy_app::App;
use bevy_ecs::prelude::*;
use parking_lot::RwLock;
use uuid::Uuid;

#[derive(Bundle, Clone)]
pub struct SimulatedPlayerBundle {
    pub position: Position,
    pub physics: Physics,
    pub physics_state: PhysicsState,
    pub look_direction: LookDirection,
    pub attributes: Attributes,
    pub inventory: InventoryComponent,
}

impl SimulatedPlayerBundle {
    pub fn new(position: Vec3) -> Self {
        let dimensions = EntityDimensions {
            width: 0.6,
            height: 1.8,
        };

        SimulatedPlayerBundle {
            position: Position::new(position),
            physics: Physics::new(dimensions, &position),
            physics_state: PhysicsState::default(),
            look_direction: LookDirection::new(0.0, 0.0),
            attributes: Attributes {
                speed: AttributeInstance::new(0.1),
                attack_speed: AttributeInstance::new(4.0),
            },
            inventory: InventoryComponent::default(),
        }
    }
}

fn simulation_instance_name() -> ResourceLocation {
    ResourceLocation::new("azalea:simulation")
}

fn create_simulation_instance(chunks: ChunkStorage) -> (App, Arc<RwLock<Instance>>) {
    let instance_name = simulation_instance_name();

    let instance = Arc::new(RwLock::new(Instance {
        chunks,
        ..Default::default()
    }));

    let mut app = App::new();
    // we don't use all the default azalea plugins because we don't need all of them
    app.add_plugins((
        azalea_physics::PhysicsPlugin,
        azalea_entity::EntityPlugin,
        azalea_client::movement::PlayerMovePlugin,
        super::PathfinderPlugin,
        crate::BotPlugin,
        azalea_client::task_pool::TaskPoolPlugin::default(),
        // for mining
        crate::auto_tool::AutoToolPlugin,
        azalea_client::inventory::InventoryPlugin,
        azalea_client::mining::MinePlugin,
        azalea_client::interact::InteractPlugin,
    ))
    .insert_resource(InstanceContainer {
        instances: [(instance_name.clone(), Arc::downgrade(&instance.clone()))]
            .iter()
            .cloned()
            .collect(),
    })
    .add_event::<SendPacketEvent>();

    app.edit_schedule(bevy_app::Main, |schedule| {
        schedule.set_executor_kind(bevy_ecs::schedule::ExecutorKind::SingleThreaded);
    });

    (app, instance)
}

fn create_simulation_player_complete_bundle(
    instance: Arc<RwLock<Instance>>,
    player: &SimulatedPlayerBundle,
) -> impl Bundle {
    let instance_name = simulation_instance_name();

    (
        MinecraftEntityId(0),
        azalea_entity::LocalEntity,
        azalea_entity::metadata::PlayerMetadataBundle::default(),
        azalea_entity::EntityBundle::new(
            Uuid::nil(),
            *player.position,
            azalea_registry::EntityKind::Player,
            instance_name,
        ),
        azalea_client::InstanceHolder {
            // partial_instance is never actually used by the pathfinder so
            partial_instance: Arc::new(RwLock::new(PartialInstance::default())),
            instance: instance.clone(),
        },
        InventoryComponent::default(),
    )
}

fn create_simulation_player(
    ecs: &mut World,
    instance: Arc<RwLock<Instance>>,
    player: SimulatedPlayerBundle,
) -> Entity {
    let mut entity = ecs.spawn(create_simulation_player_complete_bundle(instance, &player));
    entity.insert(player);
    entity.id()
}

/// Simulate the Minecraft world to see if certain movements would be possible.
pub struct Simulation {
    pub app: App,
    pub entity: Entity,
    _instance: Arc<RwLock<Instance>>,
}

impl Simulation {
    pub fn new(chunks: ChunkStorage, player: SimulatedPlayerBundle) -> Self {
        let (mut app, instance) = create_simulation_instance(chunks);
        let entity = create_simulation_player(&mut app.world, instance.clone(), player);
        Self {
            app,
            entity,
            _instance: instance,
        }
    }

    pub fn tick(&mut self) {
        self.app.update();
        self.app.world.run_schedule(GameTick);
    }
    pub fn component<T: Component + Clone>(&self) -> T {
        self.app.world.get::<T>(self.entity).unwrap().clone()
    }
    pub fn get_component<T: Component + Clone>(&self) -> Option<T> {
        self.app.world.get::<T>(self.entity).cloned()
    }
    pub fn position(&self) -> Vec3 {
        *self.component::<Position>()
    }
    pub fn is_mining(&self) -> bool {
        // return true if the component is present and Some
        self.get_component::<azalea_client::mining::MineBlockPos>()
            .map(|c| *c)
            .flatten()
            .is_some()
    }
}

/// A set of simulations, useful for efficiently doing multiple simulations.
pub struct SimulationSet {
    pub app: App,
    instance: Arc<RwLock<Instance>>,
}
impl SimulationSet {
    pub fn new(chunks: ChunkStorage) -> Self {
        let (app, instance) = create_simulation_instance(chunks);
        Self { app, instance }
    }
    pub fn tick(&mut self) {
        self.app.update();
        self.app.world.run_schedule(GameTick);
    }

    pub fn spawn(&mut self, player: SimulatedPlayerBundle) -> Entity {
        create_simulation_player(&mut self.app.world, self.instance.clone(), player)
    }
    pub fn despawn(&mut self, entity: Entity) {
        self.app.world.despawn(entity);
    }

    pub fn position(&self, entity: Entity) -> Vec3 {
        **self.app.world.get::<Position>(entity).unwrap()
    }
}
