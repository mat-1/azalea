use std::{collections::VecDeque, time::Instant};

use azalea_client::mining::FinishMiningBlockEvent;
use azalea_core::position::BlockPos;
use azalea_entity::Position;
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};

#[derive(Debug)]
pub struct RecentlyMinedBlock {
    pub block: BlockPos,
    pub time: Instant,
}

/// A component that contains the blocks that we finished mining recently. When
/// a new item is added, the ones that were added more than 5 seconds ago are
/// removed.
///
/// This is only present when the entity has the
/// [`Process`](super::process::Process) component, since it's currently only
/// used for picking up items we mined while pathfinding.
#[derive(Component, Debug, Default)]
pub struct RecentlyMinedBlocks {
    pub blocks: VecDeque<RecentlyMinedBlock>,
}

#[derive(Component, Debug, Default)]
pub struct ItemsToPickup {
    pub items: Vec<Entity>,
}

/// This is used internally to recalculate the path when there's a new item to
/// pickup.
#[derive(Component, Debug, Default)]
pub struct LastItemsToPickup {
    pub items: Vec<Entity>,
}
/// A component that tracks whether we've acknowledged the items to pickup
/// change.
///
/// This is only used internally for recalculating paths when there's a new item
/// to pick up.
#[derive(Component, Debug, Deref, DerefMut)]
pub struct ItemsToPickupChangeAcknowledged(pub bool);

pub fn add_pickup_components_to_player(
    mut commands: Commands,
    mut query: Query<Entity, Added<super::process::Process>>,
) {
    for entity in &mut query {
        commands.entity(entity).insert((
            RecentlyMinedBlocks::default(),
            ItemsToPickup::default(),
            LastItemsToPickup::default(),
            ItemsToPickupChangeAcknowledged(true),
        ));
    }
}

pub fn remove_pickup_components_from_player(
    mut commands: Commands,
    mut query: RemovedComponents<super::process::Process>,
) {
    for entity in query.read() {
        commands
            .entity(entity)
            .remove::<RecentlyMinedBlocks>()
            .remove::<ItemsToPickup>()
            .remove::<LastItemsToPickup>()
            .remove::<ItemsToPickupChangeAcknowledged>();
    }
}

pub fn watch_for_mined_blocks(
    mut finish_mining_block_events: EventReader<FinishMiningBlockEvent>,
    mut query: Query<&mut RecentlyMinedBlocks, With<super::process::Process>>,
) {
    for event in finish_mining_block_events.read() {
        let mut recently_mined_blocks = query.get_mut(event.entity).unwrap();

        // remove blocks that are too old
        let now = Instant::now();
        recently_mined_blocks
            .blocks
            .retain(|block| now.duration_since(block.time).as_secs_f32() < 5.0);

        recently_mined_blocks.blocks.push_back(RecentlyMinedBlock {
            block: event.position,
            time: now,
        });
    }
}

pub fn watch_for_item_spawns_from_blocks_we_mined(
    mut player_query: Query<(&RecentlyMinedBlocks, &Position, &mut ItemsToPickup)>,
    spawned_items_query: Query<(Entity, &Position), Added<azalea_entity::metadata::Item>>,
) {
    for (recently_mined_blocks, player_position, mut items_to_pickup) in &mut player_query {
        for (entity, position) in &mut spawned_items_query.iter() {
            if recently_mined_blocks
                .blocks
                .iter()
                .any(|block| block.block == BlockPos::from(position))
            {
                // if we're already within 1 block of the item, ignore because we probably
                // already picked it up
                if (player_position.distance_squared_to(position) < 1.0)
                    || (player_position
                        .up(player_position.y + 1.8)
                        .distance_squared_to(position)
                        < 1.0)
                {
                    // this check isn't perfect since minecraft checks with the bounding box, and
                    // the distance is different vertically, but it's good enough for our purposes
                    continue;
                }

                items_to_pickup.items.push(entity);
                println!("added item to pickup: {:?}", entity);
            }
        }
    }
}

/// Remove items from [`ItemsToPickup`] that no longer exist. This doesn't need
/// to run super frequently, so it only runs every tick.
pub fn remove_despawned_items_to_pickup(
    mut player_query: Query<&mut ItemsToPickup>,
    items_query: Query<Entity, With<azalea_entity::metadata::Item>>,
) {
    for mut items_to_pickup in &mut player_query {
        items_to_pickup
            .items
            .retain(|entity| items_query.get(*entity).is_ok());
    }
}
