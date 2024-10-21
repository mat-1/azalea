use azalea_block::{Block, BlockState};
use azalea_client::{
    inventory::{InventoryComponent, SetSelectedHotbarSlotEvent},
    mining::StartMiningBlockEvent,
    Client, InstanceHolder,
};
use azalea_core::position::BlockPos;
use azalea_entity::{update_fluid_on_eyes, FluidOnEyes, Physics};
use azalea_inventory::{ItemSlot, Menu};
use azalea_registry::{DataComponentKind, Fluid};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;

#[derive(Debug)]
pub struct BestToolResult {
    pub index: usize,
    pub percentage_per_tick: f32,
}

pub struct AutoToolPlugin;
impl Plugin for AutoToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartMiningBlockWithAutoToolEvent>()
            .add_systems(
                Update,
                start_mining_block_with_auto_tool_listener
                    .before(azalea_client::inventory::handle_set_selected_hotbar_slot_event)
                    .after(update_fluid_on_eyes)
                    .after(azalea_client::chunks::handle_receive_chunk_events),
            );
    }
}

pub trait AutoToolClientExt {
    fn best_tool_in_hotbar_for_block(&self, block: BlockState) -> BestToolResult;
}

impl AutoToolClientExt for Client {
    fn best_tool_in_hotbar_for_block(&self, block: BlockState) -> BestToolResult {
        let mut ecs = self.ecs.lock();
        let (inventory, physics, fluid_on_eyes) =
            self.query::<(&InventoryComponent, &Physics, &FluidOnEyes)>(&mut ecs);
        let menu = &inventory.inventory_menu;

        accurate_best_tool_in_hotbar_for_block(block, menu, physics, fluid_on_eyes)
    }
}

/// Returns the best tool in the hotbar for the given block.
///
/// Note that this doesn't take into account whether the player is on the ground
/// or in water, use [`accurate_best_tool_in_hotbar_for_block`] instead if you
/// care about those things.
pub fn best_tool_in_hotbar_for_block(block: BlockState, menu: &Menu) -> BestToolResult {
    accurate_best_tool_in_hotbar_for_block(
        block,
        menu,
        &Physics {
            on_ground: true,
            velocity: Default::default(),
            xxa: Default::default(),
            yya: Default::default(),
            zza: Default::default(),
            last_on_ground: Default::default(),
            dimensions: Default::default(),
            bounding_box: Default::default(),
            has_impulse: Default::default(),
            horizontal_collision: Default::default(),
            vertical_collision: Default::default(),
        },
        &FluidOnEyes::new(Fluid::Empty),
    )
}

pub fn accurate_best_tool_in_hotbar_for_block(
    block: BlockState,
    menu: &Menu,
    physics: &Physics,
    fluid_on_eyes: &FluidOnEyes,
) -> BestToolResult {
    let hotbar_slots = &menu.slots()[menu.hotbar_slots_range()];

    let mut best_speed = 0.;
    let mut best_slot = None;

    let block = Box::<dyn Block>::from(block);
    let registry_block = block.as_registry_block();

    if matches!(
        registry_block,
        azalea_registry::Block::Water | azalea_registry::Block::Lava
    ) {
        // can't mine fluids
        return BestToolResult {
            index: 0,
            percentage_per_tick: 0.,
        };
    }

    // find the first slot that has an item without durability
    for (i, item_slot) in hotbar_slots.iter().enumerate() {
        let this_item_speed;
        match item_slot {
            ItemSlot::Empty => {
                this_item_speed = Some(azalea_entity::mining::get_mine_progress(
                    block.as_ref(),
                    azalea_registry::Item::Air,
                    menu,
                    fluid_on_eyes,
                    physics,
                ));
            }
            ItemSlot::Present(item_slot) => {
                // lazy way to avoid checking durability since azalea doesn't have durability
                // data yet
                if item_slot
                    .components
                    .get(DataComponentKind::Damage)
                    .is_none()
                {
                    this_item_speed = Some(azalea_entity::mining::get_mine_progress(
                        block.as_ref(),
                        item_slot.kind,
                        menu,
                        fluid_on_eyes,
                        physics,
                    ));
                } else {
                    this_item_speed = None;
                }
            }
        }
        if let Some(this_item_speed) = this_item_speed {
            if this_item_speed > best_speed {
                best_slot = Some(i);
                best_speed = this_item_speed;
            }
        }
    }

    // now check every item
    for (i, item_slot) in hotbar_slots.iter().enumerate() {
        if let ItemSlot::Present(item_slot) = item_slot {
            let this_item_speed = azalea_entity::mining::get_mine_progress(
                block.as_ref(),
                item_slot.kind,
                menu,
                fluid_on_eyes,
                physics,
            );
            if this_item_speed > best_speed {
                best_slot = Some(i);
                best_speed = this_item_speed;
            }
        }
    }

    BestToolResult {
        index: best_slot.unwrap_or(0),
        percentage_per_tick: best_speed,
    }
}

/// An event to mine a given block, while automatically picking the best tool in
/// our hotbar to use.
#[derive(Event)]
pub struct StartMiningBlockWithAutoToolEvent {
    pub entity: Entity,
    pub position: BlockPos,
}

pub fn start_mining_block_with_auto_tool_listener(
    mut query: Query<(
        &mut InstanceHolder,
        &InventoryComponent,
        &Physics,
        &FluidOnEyes,
    )>,
    mut events: EventReader<StartMiningBlockWithAutoToolEvent>,
    mut set_selected_hotbar_slot_events: EventWriter<SetSelectedHotbarSlotEvent>,
    mut start_mining_block_events: EventWriter<StartMiningBlockEvent>,
) {
    for event in events.read() {
        let (instance_holder, inventory, physics, fluid_on_eyes) =
            query.get_mut(event.entity).unwrap();
        let instance = instance_holder.instance.read();
        let block_state = instance
            .chunks
            .get_block_state(&event.position)
            .unwrap_or_default();

        let best_tool_result = accurate_best_tool_in_hotbar_for_block(
            block_state,
            &inventory.inventory_menu,
            physics,
            fluid_on_eyes,
        );

        set_selected_hotbar_slot_events.send(SetSelectedHotbarSlotEvent {
            entity: event.entity,
            slot: best_tool_result.index as u8,
        });
        start_mining_block_events.send(StartMiningBlockEvent {
            entity: event.entity,
            position: event.position,
        });
    }
}
