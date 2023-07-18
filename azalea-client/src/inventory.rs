use std::collections::{HashMap, HashSet};

use azalea_chat::FormattedText;
pub use azalea_inventory::*;
use azalea_inventory::{
    item::MaxStackSizeExt,
    operations::{
        ClickOperation, CloneClick, PickupAllClick, PickupClick, QuickCraftKind, QuickCraftStatus,
        QuickCraftStatusKind, QuickMoveClick, ThrowClick,
    },
};
use azalea_protocol::packets::game::{
    serverbound_container_click_packet::ServerboundContainerClickPacket,
    serverbound_container_close_packet::ServerboundContainerClosePacket,
};
use azalea_registry::MenuKind;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::EventReader,
    prelude::{Event, EventWriter},
    schedule::{IntoSystemConfigs, SystemSet},
    system::Query,
};
use log::warn;

use crate::{client::PlayerAbilities, local_player::handle_send_packet_event, Client, LocalPlayer};

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClientSideCloseContainerEvent>()
            .add_event::<MenuOpenedEvent>()
            .add_event::<CloseContainerEvent>()
            .add_event::<ContainerClickEvent>()
            .add_event::<SetContainerContentEvent>()
            .add_systems(
                Update,
                (
                    handle_menu_opened_event,
                    handle_set_container_content_event,
                    handle_container_click_event,
                    handle_container_close_event.before(handle_send_packet_event),
                    handle_client_side_close_container_event,
                )
                    .chain()
                    .in_set(InventorySet),
            );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct InventorySet;

impl Client {
    /// Return the menu that is currently open. If no menu is open, this will
    /// have the player's inventory.
    pub fn menu(&self) -> Menu {
        let mut ecs = self.ecs.lock();
        let inventory = self.query::<&InventoryComponent>(&mut ecs);
        inventory.menu().clone()
    }
}

/// A component present on all local players that have an inventory.
#[derive(Component, Debug)]
pub struct InventoryComponent {
    /// A component that contains the player's inventory menu. This is
    /// guaranteed to be a `Menu::Player`.
    ///
    /// We keep it as a [`Menu`] since `Menu` has some useful functions that
    /// bare [`azalea_inventory::Player`] doesn't have.
    pub inventory_menu: azalea_inventory::Menu,

    /// The ID of the container that's currently open. Its value is not
    /// guaranteed to be anything specific, and may change every time you open a
    /// container (unless it's 0, in which case it means that no container is
    /// open).
    pub id: u8,
    /// The current container menu that the player has open. If no container is
    /// open, this will be `None`.
    pub container_menu: Option<azalea_inventory::Menu>,
    /// The item that is currently held by the cursor. `Slot::Empty` if nothing
    /// is currently being held.
    ///
    /// This is different from [`Self::selected_hotbar_slot`], which is the
    /// item that's selected in the hotbar.
    pub carried: ItemSlot,
    /// An identifier used by the server to track client inventory desyncs. This
    /// is sent on every container click, and it's only ever updated when the
    /// server sends a new container update.
    pub state_id: u32,

    pub quick_craft_status: QuickCraftStatusKind,
    pub quick_craft_kind: QuickCraftKind,
    /// A set of the indexes of the slots that have been right clicked in
    /// this "quick craft".
    pub quick_craft_slots: HashSet<u16>,

    /// The index of the item in the hotbar that's currently being held by the
    /// player. This MUST be in the range 0..9 (not including 9).
    ///
    /// In a vanilla client this is changed by pressing the number keys or using
    /// the scroll wheel.
    pub selected_hotbar_slot: u8,
}

impl InventoryComponent {
    /// Returns a reference to the currently active menu. If a container is open
    /// it'll return [`Self::container_menu`], otherwise
    /// [`Self::inventory_menu`].
    ///
    /// Use [`Self::menu_mut`] if you need a mutable reference.
    pub fn menu(&self) -> &azalea_inventory::Menu {
        if let Some(menu) = &self.container_menu {
            menu
        } else {
            &self.inventory_menu
        }
    }

    /// Returns a mutable reference to the currently active menu. If a container
    /// is open it'll return [`Self::container_menu`], otherwise
    /// [`Self::inventory_menu`].
    ///
    /// Use [`Self::menu`] if you don't need a mutable reference.
    pub fn menu_mut(&mut self) -> &mut azalea_inventory::Menu {
        if let Some(menu) = &mut self.container_menu {
            menu
        } else {
            &mut self.inventory_menu
        }
    }

    /// Modify the inventory as if the given operation was performed on it.
    pub fn simulate_click(
        &mut self,
        operation: &ClickOperation,
        player_abilities: &PlayerAbilities,
    ) {
        if let ClickOperation::QuickCraft(quick_craft) = operation {
            let last_quick_craft_status_tmp = self.quick_craft_status.clone();
            self.quick_craft_status = last_quick_craft_status_tmp.clone();
            let last_quick_craft_status = last_quick_craft_status_tmp;

            // no carried item, reset
            if self.carried.is_empty() {
                return self.reset_quick_craft();
            }
            // if we were starting or ending, or now we aren't ending and the status
            // changed, reset
            if (last_quick_craft_status == QuickCraftStatusKind::Start
                || last_quick_craft_status == QuickCraftStatusKind::End
                || self.quick_craft_status != QuickCraftStatusKind::End)
                && (self.quick_craft_status != last_quick_craft_status)
            {
                return self.reset_quick_craft();
            }
            if self.quick_craft_status == QuickCraftStatusKind::Start {
                self.quick_craft_kind = quick_craft.kind.clone();
                if self.quick_craft_kind == QuickCraftKind::Middle && player_abilities.instant_break
                {
                    self.quick_craft_status = QuickCraftStatusKind::Add;
                    self.quick_craft_slots.clear();
                } else {
                    self.reset_quick_craft();
                }
                return;
            }
            if let QuickCraftStatus::Add { slot } = quick_craft.status {
                let slot_item = self.menu().slot(slot as usize);
                if let Some(slot_item) = slot_item {
                    if let ItemSlot::Present(carried) = &self.carried {
                        // minecraft also checks slot.may_place(carried) and
                        // menu.can_drag_to(slot)
                        // but they always return true so they're not relevant for us
                        if can_item_quick_replace(slot_item, &self.carried, true)
                            && (self.quick_craft_kind == QuickCraftKind::Right
                                || carried.count as usize > self.quick_craft_slots.len())
                        {
                            self.quick_craft_slots.insert(slot);
                        }
                    }
                }
                return;
            }
            if self.quick_craft_status == QuickCraftStatusKind::End {
                if !self.quick_craft_slots.is_empty() {
                    if self.quick_craft_slots.len() == 1 {
                        // if we only clicked one slot, then turn this
                        // QuickCraftClick into a PickupClick
                        let slot = *self.quick_craft_slots.iter().next().unwrap();
                        self.reset_quick_craft();
                        self.simulate_click(
                            &match self.quick_craft_kind {
                                QuickCraftKind::Left => {
                                    PickupClick::Left { slot: Some(slot) }.into()
                                }
                                QuickCraftKind::Right => {
                                    PickupClick::Left { slot: Some(slot) }.into()
                                }
                                QuickCraftKind::Middle => {
                                    // idk just do nothing i guess
                                    return;
                                }
                            },
                            player_abilities,
                        );
                        return;
                    }

                    let ItemSlot::Present(mut carried) = self.carried.clone() else {
                        // this should never happen
                        return self.reset_quick_craft();
                    };

                    let mut carried_count = carried.count;
                    let mut quick_craft_slots_iter = self.quick_craft_slots.iter();

                    loop {
                        let mut slot: &ItemSlot;
                        let mut slot_index: u16;
                        let mut item_stack: &ItemSlot;

                        loop {
                            let Some(&next_slot) = quick_craft_slots_iter.next() else {
                                carried.count = carried_count;
                                self.carried = ItemSlot::Present(carried);
                                return self.reset_quick_craft();
                            };

                            slot = self.menu().slot(next_slot as usize).unwrap();
                            slot_index = next_slot;
                            item_stack = &self.carried;

                            if slot.is_present()
                                    && can_item_quick_replace(slot, item_stack, true)
                                    // this always returns true in most cases
                                    // && slot.may_place(item_stack)
                                    && (
                                        self.quick_craft_kind == QuickCraftKind::Middle
                                        || item_stack.count() as i32 >= self.quick_craft_slots.len() as i32
                                    )
                            {
                                break;
                            }
                        }

                        // get the ItemSlotData for the slot
                        let ItemSlot::Present(slot) = slot else {
                            unreachable!("the loop above requires the slot to be present to break")
                        };

                        // if self.can_drag_to(slot) {
                        let mut new_carried = carried.clone();
                        let slot_item_count = slot.count;
                        get_quick_craft_slot_count(
                            &self.quick_craft_slots,
                            &self.quick_craft_kind,
                            &mut new_carried,
                            slot_item_count,
                        );
                        let max_stack_size = i8::min(
                            new_carried.kind.max_stack_size(),
                            i8::min(
                                new_carried.kind.max_stack_size(),
                                slot.kind.max_stack_size(),
                            ),
                        );
                        if new_carried.count > max_stack_size {
                            new_carried.count = max_stack_size;
                        }

                        carried_count -= new_carried.count - slot_item_count;
                        // we have to inline self.menu_mut() here to avoid the borrow checker
                        // complaining
                        let menu = if let Some(menu) = &mut self.container_menu {
                            menu
                        } else {
                            &mut self.inventory_menu
                        };
                        *menu.slot_mut(slot_index as usize).unwrap() =
                            ItemSlot::Present(new_carried);
                    }
                }
            } else {
                return self.reset_quick_craft();
            }
        }
        // the quick craft status should always be in start if we're not in quick craft
        // mode
        if self.quick_craft_status != QuickCraftStatusKind::Start {
            return self.reset_quick_craft();
        }

        match operation {
            // left clicking outside inventory
            ClickOperation::Pickup(PickupClick::Left { slot: None }) => {
                if self.carried.is_present() {
                    // vanilla has `player.drop`s but they're only used
                    // server-side
                    // they're included as comments here in case you want to adapt this for a server
                    // implementation

                    // player.drop(self.carried, true);
                    self.carried = ItemSlot::Empty;
                }
            }
            ClickOperation::Pickup(PickupClick::Right { slot: None }) => {
                if self.carried.is_present() {
                    let _item = self.carried.split(1);
                    // player.drop(item, true);
                }
            }
            ClickOperation::Pickup(
                PickupClick::Left { slot: Some(slot) } | PickupClick::Right { slot: Some(slot) },
            ) => {
                let Some(slot_item) = self.menu().slot(*slot as usize) else {
                    return;
                };
                let carried = &self.carried;
                // vanilla does a check called tryItemClickBehaviourOverride
                // here
                // i don't understand it so i didn't implement it
                match slot_item {
                    ItemSlot::Empty => if carried.is_present() {},
                    ItemSlot::Present(_) => todo!(),
                }
            }
            ClickOperation::QuickMove(
                QuickMoveClick::Left { slot } | QuickMoveClick::Right { slot },
            ) => {
                // in vanilla it also tests if QuickMove has a slot index of -999
                // but i don't think that's ever possible so it's not covered here
                loop {
                    let new_slot_item = self.menu_mut().quick_move_stack(*slot as usize);
                    let slot_item = self.menu().slot(*slot as usize).unwrap();
                    if new_slot_item.is_empty() || slot_item != &new_slot_item {
                        break;
                    }
                }
            }
            ClickOperation::Swap(s) => {
                let source_slot_index = s.source_slot as usize;
                let target_slot_index = s.target_slot as usize;

                let Some(source_slot) = self.menu().slot(source_slot_index) else {
                    return;
                };
                let Some(target_slot) = self.menu().slot(target_slot_index) else {
                    return;
                };
                if source_slot.is_empty() && target_slot.is_empty() {
                    return;
                }

                if target_slot.is_empty() {
                    if self.menu().may_pickup(source_slot_index) {
                        let source_slot = source_slot.clone();
                        let target_slot = self.menu_mut().slot_mut(target_slot_index).unwrap();
                        *target_slot = source_slot;
                    }
                } else if source_slot.is_empty() {
                    let ItemSlot::Present(target_item) = target_slot else {
                        unreachable!("target slot is not empty but is not present");
                    };
                    if self.menu().may_place(source_slot_index, target_item) {
                        // get the target_item but mutable
                        let source_max_stack_size = self.menu().max_stack_size(source_slot_index);

                        let target_slot = self.menu_mut().slot_mut(target_slot_index).unwrap();
                        let new_source_slot = target_slot.split(source_max_stack_size);
                        *self.menu_mut().slot_mut(source_slot_index).unwrap() = new_source_slot;
                    }
                } else if self.menu().may_pickup(source_slot_index) {
                    let ItemSlot::Present(target_item) = target_slot else {
                        unreachable!("target slot is not empty but is not present");
                    };
                    if self.menu().may_place(source_slot_index, target_item) {
                        let source_max_stack = self.menu().max_stack_size(source_slot_index);
                        if target_slot.count() > source_max_stack as i8 {
                            // if there's more than the max stack size in the target slot

                            let target_slot = self.menu_mut().slot_mut(target_slot_index).unwrap();
                            let new_source_slot = target_slot.split(source_max_stack);
                            *self.menu_mut().slot_mut(source_slot_index).unwrap() = new_source_slot;
                            // if !self.inventory_menu.add(new_source_slot) {
                            //     player.drop(new_source_slot, true);
                            // }
                        } else {
                            // normal swap
                            let new_target_slot = source_slot.clone();
                            let new_source_slot = target_slot.clone();

                            let target_slot = self.menu_mut().slot_mut(target_slot_index).unwrap();
                            *target_slot = new_target_slot;

                            let source_slot = self.menu_mut().slot_mut(source_slot_index).unwrap();
                            *source_slot = new_source_slot;
                        }
                    }
                }
            }
            ClickOperation::Clone(CloneClick { slot }) => {
                if !player_abilities.instant_break || self.carried.is_present() {
                    return;
                }
                let Some(source_slot) = self.menu().slot(*slot as usize) else {
                    return;
                };
                let ItemSlot::Present(source_item) = source_slot else {
                    return;
                };
                let mut new_carried = source_item.clone();
                new_carried.count = new_carried.kind.max_stack_size();
                self.carried = ItemSlot::Present(new_carried);
            }
            ClickOperation::Throw(c) => {
                if self.carried.is_present() {
                    return;
                }

                let (ThrowClick::Single { slot: slot_index }
                | ThrowClick::All { slot: slot_index }) = c;
                let slot_index = *slot_index as usize;

                let Some(slot) = self.menu_mut().slot_mut(slot_index) else {
                    return;
                };
                let ItemSlot::Present(slot_item) = slot else {
                    return;
                };

                let dropping_count = match c {
                    ThrowClick::Single { .. } => 1,
                    ThrowClick::All { .. } => slot_item.count,
                };

                let _dropping = slot_item.split(dropping_count as u8);
                // player.drop(dropping, true);
            }
            ClickOperation::PickupAll(PickupAllClick {
                slot: source_slot_index,
                reversed,
            }) => {
                let source_slot_index = *source_slot_index as usize;

                let source_slot = self.menu().slot(source_slot_index).unwrap();
                let target_slot = self.carried.clone();

                if target_slot.is_empty()
                    || (source_slot.is_present() && self.menu().may_pickup(source_slot_index))
                {
                    return;
                }

                let ItemSlot::Present(target_slot_item) = &target_slot else {
                    unreachable!("target slot is not empty but is not present");
                };

                for round in 0..2 {
                    let iterator: Box<dyn Iterator<Item = usize>> = if *reversed {
                        Box::new((0..self.menu().len()).rev())
                    } else {
                        Box::new(0..self.menu().len())
                    };

                    for i in iterator {
                        if target_slot_item.count < target_slot_item.kind.max_stack_size() {
                            let checking_slot = self.menu().slot(i).unwrap();
                            if let ItemSlot::Present(checking_item) = checking_slot {
                                if can_item_quick_replace(checking_slot, &target_slot, true)
                                    && self.menu().may_pickup(i)
                                    && (round != 0
                                        || checking_item.count
                                            != checking_item.kind.max_stack_size())
                                {
                                    // get the checking_slot and checking_item again but mutable
                                    let checking_slot = self.menu_mut().slot_mut(i).unwrap();

                                    let taken_item =
                                        checking_slot.split(checking_slot.count() as u8);

                                    // now extend the carried item
                                    let target_slot = &mut self.carried;
                                    let ItemSlot::Present(target_slot_item) = target_slot else {
                                        unreachable!("target slot is not empty but is not present");
                                    };
                                    target_slot_item.count += taken_item.count();
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn reset_quick_craft(&mut self) {
        self.quick_craft_status = QuickCraftStatusKind::Start;
        self.quick_craft_slots.clear();
    }

    /// Get the item in the player's hotbar that is currently being held.
    pub fn held_item(&self) -> ItemSlot {
        let inventory = &self.inventory_menu;
        let hotbar_items = &inventory.slots()[inventory.hotbar_slots_range()];
        hotbar_items[self.selected_hotbar_slot as usize].clone()
    }
}

fn can_item_quick_replace(
    target_slot: &ItemSlot,
    item: &ItemSlot,
    ignore_item_count: bool,
) -> bool {
    let ItemSlot::Present(target_slot) = target_slot else {
        return false;
    };
    let ItemSlot::Present(item) = item else {
        // i *think* this is what vanilla does
        // not 100% sure lol probably doesn't matter though
        return false;
    };

    if !item.is_same_item_and_nbt(target_slot) {
        return false;
    }
    let count = target_slot.count as u16
        + if ignore_item_count {
            0
        } else {
            item.count as u16
        };
    count <= item.kind.max_stack_size() as u16
}

fn get_quick_craft_slot_count(
    quick_craft_slots: &HashSet<u16>,
    quick_craft_kind: &QuickCraftKind,
    item: &mut ItemSlotData,
    slot_item_count: i8,
) {
    item.count = match quick_craft_kind {
        QuickCraftKind::Left => item.count / quick_craft_slots.len() as i8,
        QuickCraftKind::Right => 1,
        QuickCraftKind::Middle => item.kind.max_stack_size(),
    };
    item.count += slot_item_count;
}

impl Default for InventoryComponent {
    fn default() -> Self {
        InventoryComponent {
            inventory_menu: Menu::Player(azalea_inventory::Player::default()),
            id: 0,
            container_menu: None,
            carried: ItemSlot::Empty,
            state_id: 0,
            quick_craft_status: QuickCraftStatusKind::Start,
            quick_craft_kind: QuickCraftKind::Middle,
            quick_craft_slots: HashSet::new(),
            selected_hotbar_slot: 0,
        }
    }
}

/// Sent from the server when a menu (like a chest or crafting table) was
/// opened by the client.
#[derive(Event, Debug)]
pub struct MenuOpenedEvent {
    pub entity: Entity,
    pub window_id: u32,
    pub menu_type: MenuKind,
    pub title: FormattedText,
}
fn handle_menu_opened_event(
    mut events: EventReader<MenuOpenedEvent>,
    mut query: Query<&mut InventoryComponent>,
) {
    for event in events.iter() {
        let mut inventory = query.get_mut(event.entity).unwrap();
        inventory.id = event.window_id as u8;
        inventory.container_menu = Some(Menu::from_kind(event.menu_type));
    }
}

/// Tell the server that we want to close a container.
///
/// Note that this is also sent when the client closes its own inventory, even
/// though there is no packet for opening its inventory.
#[derive(Event)]
pub struct CloseContainerEvent {
    pub entity: Entity,
    /// The ID of the container to close. 0 for the player's inventory. If this
    /// is not the same as the currently open inventory, nothing will happen.
    pub id: u8,
}
fn handle_container_close_event(
    mut events: EventReader<CloseContainerEvent>,
    mut client_side_events: EventWriter<ClientSideCloseContainerEvent>,
    query: Query<(&LocalPlayer, &InventoryComponent)>,
) {
    for event in events.iter() {
        let (local_player, inventory) = query.get(event.entity).unwrap();
        if event.id != inventory.id {
            warn!(
                "Tried to close container with ID {}, but the current container ID is {}",
                event.id, inventory.id
            );
            continue;
        }

        local_player.write_packet(
            ServerboundContainerClosePacket {
                container_id: inventory.id,
            }
            .get(),
        );
        client_side_events.send(ClientSideCloseContainerEvent {
            entity: event.entity,
        });
    }
}

/// Close a container without notifying the server.
///
/// Note that this also gets fired when we get a [`CloseContainerEvent`].
#[derive(Event)]
pub struct ClientSideCloseContainerEvent {
    pub entity: Entity,
}
pub fn handle_client_side_close_container_event(
    mut events: EventReader<ClientSideCloseContainerEvent>,
    mut query: Query<&mut InventoryComponent>,
) {
    for event in events.iter() {
        let mut inventory = query.get_mut(event.entity).unwrap();
        inventory.container_menu = None;
        inventory.id = 0;
    }
}

#[derive(Event, Debug)]
pub struct ContainerClickEvent {
    pub entity: Entity,
    pub window_id: u8,
    pub operation: ClickOperation,
}
pub fn handle_container_click_event(
    mut events: EventReader<ContainerClickEvent>,
    mut query: Query<(&mut InventoryComponent, &LocalPlayer)>,
) {
    for event in events.iter() {
        let (mut inventory, local_player) = query.get_mut(event.entity).unwrap();
        if inventory.id != event.window_id {
            warn!(
                "Tried to click container with ID {}, but the current container ID is {}",
                event.window_id, inventory.id
            );
            continue;
        }

        let menu = inventory.menu_mut();
        let old_slots = menu.slots().clone();

        // menu.click(&event.operation);

        // see which slots changed after clicking and put them in the hashmap
        // the server uses this to check if we desynced
        let mut changed_slots: HashMap<u16, ItemSlot> = HashMap::new();
        for (slot_index, old_slot) in old_slots.iter().enumerate() {
            let new_slot = &menu.slots()[slot_index];
            if old_slot != new_slot {
                changed_slots.insert(slot_index as u16, new_slot.clone());
            }
        }

        local_player.write_packet(
            ServerboundContainerClickPacket {
                container_id: event.window_id,
                state_id: inventory.state_id,
                slot_num: event.operation.slot_num().map(|n| n as i16).unwrap_or(-999),
                button_num: event.operation.button_num(),
                click_type: event.operation.click_type(),
                changed_slots,
                carried_item: inventory.carried.clone(),
            }
            .get(),
        )
    }
}

/// Sent from the server when the contents of a container are replaced. Usually
/// triggered by the `ContainerSetContent` packet.
#[derive(Event)]
pub struct SetContainerContentEvent {
    pub entity: Entity,
    pub slots: Vec<ItemSlot>,
    pub container_id: u8,
}
fn handle_set_container_content_event(
    mut events: EventReader<SetContainerContentEvent>,
    mut query: Query<&mut InventoryComponent>,
) {
    for event in events.iter() {
        let mut inventory = query.get_mut(event.entity).unwrap();

        if event.container_id != inventory.id {
            warn!(
                "Tried to set container content with ID {}, but the current container ID is {}",
                event.container_id, inventory.id
            );
            continue;
        }

        let menu = inventory.menu_mut();
        for (i, slot) in event.slots.iter().enumerate() {
            if let Some(slot_mut) = menu.slot_mut(i) {
                *slot_mut = slot.clone();
            }
        }
    }
}
