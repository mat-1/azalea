pub mod mine_area;
pub mod mine_forever;

use azalea_client::{mining::Mining, InstanceHolder};
use azalea_entity::Position;

use crate::{
    auto_tool::StartMiningBlockWithAutoToolEvent,
    ecs::prelude::*,
    pathfinder::{self, ExecutingPath, GotoEvent, Pathfinder},
    LookAtEvent,
};

use super::pickup::{ItemsToPickup, ItemsToPickupChangeAcknowledged, LastItemsToPickup};

#[derive(Component, Clone, Debug)]
pub enum Process {
    MineArea(mine_area::MineArea),
    MineForever(mine_forever::MineForever),
}

impl From<mine_area::MineArea> for Process {
    fn from(mine_area: mine_area::MineArea) -> Self {
        Self::MineArea(mine_area)
    }
}
impl From<mine_forever::MineForever> for Process {
    fn from(mine_forever: mine_forever::MineForever) -> Self {
        Self::MineForever(mine_forever)
    }
}

#[derive(Event)]
pub struct SetActiveProcessEvent {
    pub entity: Entity,
    pub process: Process,
}

pub fn set_active_pathfinder_process_listener(
    mut commands: Commands,
    mut events: EventReader<SetActiveProcessEvent>,
    mut stop_pathfinding_events: EventWriter<pathfinder::StopPathfindingEvent>,
) {
    for event in events.read() {
        stop_pathfinding_events.send(pathfinder::StopPathfindingEvent {
            entity: event.entity,
            force: false,
        });
        commands.entity(event.entity).insert(event.process.clone());
    }
}

pub struct ProcessSystemComponents<'a> {
    pub entity: Entity,
    pub position: &'a Position,
    pub instance_holder: &'a InstanceHolder,
    pub pathfinder: &'a Pathfinder,
    pub items_to_pickup_change_acknowledged: Mut<'a, ItemsToPickupChangeAcknowledged>,
    pub mining: Option<&'a Mining>,
    pub executing_path: Option<&'a ExecutingPath>,
}

#[allow(clippy::type_complexity)]
pub fn process_tick(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Process,
        &Position,
        &InstanceHolder,
        &Pathfinder,
        &ItemsToPickup,
        &mut LastItemsToPickup,
        &mut ItemsToPickupChangeAcknowledged,
        Option<&Mining>,
        Option<&ExecutingPath>,
    )>,
    position_query: Query<&Position>,
    mut goto_events: EventWriter<GotoEvent>,
    mut look_at_events: EventWriter<LookAtEvent>,
    mut start_mining_block_events: EventWriter<StartMiningBlockWithAutoToolEvent>,
) {
    for (
        entity,
        process,
        position,
        instance_holder,
        pathfinder,
        items_to_pickup,
        mut last_items_to_pickup,
        mut items_to_pickup_change_acknowledged,
        mining,
        executing_path,
    ) in &mut query
    {
        let items_to_pickup_positions = items_to_pickup
            .items
            .iter()
            .filter_map(|&e| position_query.get(e).ok())
            .map(|p| **p)
            .collect::<Vec<_>>();
        // if there's any item in items_to_pickup that isn't in last_items_to_pickup
        let is_items_to_pickup_changed = items_to_pickup
            .items
            .iter()
            .any(|&e| !last_items_to_pickup.items.contains(&e));
        if is_items_to_pickup_changed {
            **items_to_pickup_change_acknowledged = false;
            last_items_to_pickup.items = items_to_pickup.items.clone();
        }

        let components = ProcessSystemComponents {
            entity,
            position,
            instance_holder,
            pathfinder,
            items_to_pickup_change_acknowledged,
            mining,
            executing_path,
        };
        match process {
            Process::MineArea(mine_area) => {
                mine_area::mine_area(
                    mine_area,
                    &mut commands,
                    components,
                    &mut goto_events,
                    &mut look_at_events,
                    &mut start_mining_block_events,
                );
            }
            Process::MineForever(mine_forever) => {
                mine_forever::mine_forever(
                    mine_forever,
                    &mut commands,
                    components,
                    &items_to_pickup_positions,
                    &mut goto_events,
                    &mut look_at_events,
                    &mut start_mining_block_events,
                );
            }
        }
    }
}
