use azalea_buf::McBuf;

use crate::{ItemSlot, ItemSlotData, Menu, MenuLocation, Player, PlayerMenuLocation};

#[derive(Debug, Clone)]
pub enum ClickOperation {
    Pickup(PickupClick),
    QuickMove(QuickMoveClick),
    Swap(SwapClick),
    Clone(CloneClick),
    Throw(ThrowClick),
    QuickCraft(QuickCraftClick),
    PickupAll(PickupAllClick),
}

#[derive(Debug, Clone)]
pub enum PickupClick {
    /// Left mouse click. Note that in the protocol, None is represented as
    /// -999.
    Left { slot: Option<u16> },
    /// Right mouse click. Note that in the protocol, None is represented as
    /// -999.
    Right { slot: Option<u16> },
    /// Drop cursor stack.
    LeftOutside,
    /// Drop cursor single item.
    RightOutside,
}
impl From<PickupClick> for ClickOperation {
    fn from(click: PickupClick) -> Self {
        ClickOperation::Pickup(click)
    }
}

/// Shift click
#[derive(Debug, Clone)]
pub enum QuickMoveClick {
    /// Shift + left mouse click
    Left { slot: u16 },
    /// Shift + right mouse click (identical behavior)
    Right { slot: u16 },
}
impl From<QuickMoveClick> for ClickOperation {
    fn from(click: QuickMoveClick) -> Self {
        ClickOperation::QuickMove(click)
    }
}
#[derive(Debug, Clone)]
pub enum SwapClick {
    /// Like number keys 1-9 (but 0 indexed so actually 0-8)
    Hotbar { slot: u16, index: u8 },
    /// Offhand swap key F
    Offhand { slot: u16 },
}
impl From<SwapClick> for ClickOperation {
    fn from(click: SwapClick) -> Self {
        ClickOperation::Swap(click)
    }
}
/// Middle click, only defined for creative players in non-player
/// inventories.
#[derive(Debug, Clone)]
pub struct CloneClick {
    pub slot: u16,
}
impl From<CloneClick> for ClickOperation {
    fn from(click: CloneClick) -> Self {
        ClickOperation::Clone(click)
    }
}
#[derive(Debug, Clone)]
pub enum ThrowClick {
    /// Drop key (Q)
    Single { slot: u16 },
    /// Ctrl + drop key (Q)
    All { slot: u16 },
}
impl From<ThrowClick> for ClickOperation {
    fn from(click: ThrowClick) -> Self {
        ClickOperation::Throw(click)
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuickCraftClick {
    pub kind: QuickCraftKind,
    pub status: QuickCraftStatus,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum QuickCraftKind {
    Left,
    Right,
    Middle,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum QuickCraftStatusKind {
    /// Starting drag
    Start,
    /// Add slot
    Add,
    /// Ending drag
    End,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum QuickCraftStatus {
    /// Starting drag
    Start,
    /// Add a slot.
    Add { slot: u16 },
    /// Ending drag
    End,
}
impl From<QuickCraftStatus> for QuickCraftStatusKind {
    fn from(status: QuickCraftStatus) -> Self {
        match status {
            QuickCraftStatus::Start => QuickCraftStatusKind::Start,
            QuickCraftStatus::Add { .. } => QuickCraftStatusKind::Add,
            QuickCraftStatus::End => QuickCraftStatusKind::End,
        }
    }
}

/// Double click
#[derive(Debug, Clone)]
pub struct PickupAllClick {
    pub slot: u16,
}
impl From<PickupAllClick> for ClickOperation {
    fn from(click: PickupAllClick) -> Self {
        ClickOperation::PickupAll(click)
    }
}

impl ClickOperation {
    /// Return the slot number that this operation is acting on, if any.
    ///
    /// Note that in the protocol, "None" is represented as -999.
    pub fn slot_num(&self) -> Option<u16> {
        match self {
            ClickOperation::Pickup(pickup) => match pickup {
                PickupClick::Left { slot } => *slot,
                PickupClick::Right { slot } => *slot,
                PickupClick::LeftOutside => None,
                PickupClick::RightOutside => None,
            },
            ClickOperation::QuickMove(quick_move) => match quick_move {
                QuickMoveClick::Left { slot } => Some(*slot),
                QuickMoveClick::Right { slot } => Some(*slot),
            },
            ClickOperation::Swap(swap) => match swap {
                SwapClick::Hotbar { slot, .. } => Some(*slot),
                SwapClick::Offhand { slot } => Some(*slot),
            },
            ClickOperation::Clone(clone) => Some(clone.slot),
            ClickOperation::Throw(throw) => match throw {
                ThrowClick::Single { slot } => Some(*slot),
                ThrowClick::All { slot } => Some(*slot),
            },
            ClickOperation::QuickCraft(quick_craft) => match quick_craft.status {
                QuickCraftStatus::Start => None,
                QuickCraftStatus::Add { slot } => Some(slot),
                QuickCraftStatus::End => None,
            },
            ClickOperation::PickupAll(pickup_all) => Some(pickup_all.slot),
        }
    }

    pub fn button_num(&self) -> u8 {
        match self {
            ClickOperation::Pickup(pickup) => match pickup {
                PickupClick::Left { .. } => 0,
                PickupClick::Right { .. } => 1,
                PickupClick::LeftOutside => 0,
                PickupClick::RightOutside => 1,
            },
            ClickOperation::QuickMove(quick_move) => match quick_move {
                QuickMoveClick::Left { .. } => 0,
                QuickMoveClick::Right { .. } => 1,
            },
            ClickOperation::Swap(swap) => match swap {
                SwapClick::Hotbar { index, .. } => *index,
                SwapClick::Offhand { .. } => 40,
            },
            ClickOperation::Clone(_) => 2,
            ClickOperation::Throw(throw) => match throw {
                ThrowClick::Single { .. } => 0,
                ThrowClick::All { .. } => 1,
            },
            ClickOperation::QuickCraft(quick_craft) => match quick_craft {
                QuickCraftClick {
                    kind: QuickCraftKind::Left,
                    status: QuickCraftStatus::Start,
                } => 0,
                QuickCraftClick {
                    kind: QuickCraftKind::Right,
                    status: QuickCraftStatus::Start,
                } => 4,
                QuickCraftClick {
                    kind: QuickCraftKind::Middle,
                    status: QuickCraftStatus::Start,
                } => 8,
                QuickCraftClick {
                    kind: QuickCraftKind::Left,
                    status: QuickCraftStatus::Add { .. },
                } => 1,
                QuickCraftClick {
                    kind: QuickCraftKind::Right,
                    status: QuickCraftStatus::Add { .. },
                } => 5,
                QuickCraftClick {
                    kind: QuickCraftKind::Middle,
                    status: QuickCraftStatus::Add { .. },
                } => 9,
                QuickCraftClick {
                    kind: QuickCraftKind::Left,
                    status: QuickCraftStatus::End,
                } => 2,
                QuickCraftClick {
                    kind: QuickCraftKind::Right,
                    status: QuickCraftStatus::End,
                } => 6,
                QuickCraftClick {
                    kind: QuickCraftKind::Middle,
                    status: QuickCraftStatus::End,
                } => 10,
            },
            ClickOperation::PickupAll(_) => 0,
        }
    }

    pub fn click_type(&self) -> ClickType {
        match self {
            ClickOperation::Pickup(_) => ClickType::Pickup,
            ClickOperation::QuickMove(_) => ClickType::QuickMove,
            ClickOperation::Swap(_) => ClickType::Swap,
            ClickOperation::Clone(_) => ClickType::Clone,
            ClickOperation::Throw(_) => ClickType::Throw,
            ClickOperation::QuickCraft(_) => ClickType::QuickCraft,
            ClickOperation::PickupAll(_) => ClickType::PickupAll,
        }
    }
}

#[derive(McBuf, Clone, Copy, Debug)]
pub enum ClickType {
    Pickup = 0,
    QuickMove = 1,
    Swap = 2,
    Clone = 3,
    Throw = 4,
    QuickCraft = 5,
    PickupAll = 6,
}

impl Menu {
    /// Shift-click a slot in this menu.
    pub fn quick_move_stack(&mut self, slot_index: usize) -> ItemSlot {
        let slot = self.slot(slot_index as usize);
        let Some(ItemSlot::Present(slot)) = slot else {
            return ItemSlot::Empty;
        };

        let original_slot_item = slot;

        let slot_location = self
            .location_for_slot(slot_index)
            .expect("we just checked to make sure the slot is Some above, so this shouldn't be able to error");
        match slot_location {
            MenuLocation::Player(l) => {
                let Menu::Player(menu) = self else {
                    unreachable!()
                };
                match l {
                    PlayerMenuLocation::CraftResult => {
                        move_item_stack_to(slot, menu.craft_result, true)
                    }
                    PlayerMenuLocation::Craft => todo!(),
                    PlayerMenuLocation::Armor => todo!(),
                    PlayerMenuLocation::Inventory => todo!(),
                    PlayerMenuLocation::Offhand => todo!(),
                }
            }
            MenuLocation::Generic9x1(_) => todo!(),
            MenuLocation::Generic9x2(_) => todo!(),
            MenuLocation::Generic9x3(_) => todo!(),
            MenuLocation::Generic9x4(_) => todo!(),
            MenuLocation::Generic9x5(_) => todo!(),
            MenuLocation::Generic9x6(_) => todo!(),
            MenuLocation::Generic3x3(_) => todo!(),
            MenuLocation::Anvil(_) => todo!(),
            MenuLocation::Beacon(_) => todo!(),
            MenuLocation::BlastFurnace(_) => todo!(),
            MenuLocation::BrewingStand(_) => todo!(),
            MenuLocation::Crafting(_) => todo!(),
            MenuLocation::Enchantment(_) => todo!(),
            MenuLocation::Furnace(_) => todo!(),
            MenuLocation::Grindstone(_) => todo!(),
            MenuLocation::Hopper(_) => todo!(),
            MenuLocation::Lectern(_) => todo!(),
            MenuLocation::Loom(_) => todo!(),
            MenuLocation::Merchant(_) => todo!(),
            MenuLocation::ShulkerBox(_) => todo!(),
            MenuLocation::LegacySmithing(_) => todo!(),
            MenuLocation::Smithing(_) => todo!(),
            MenuLocation::Smoker(_) => todo!(),
            MenuLocation::CartographyTable(_) => todo!(),
            MenuLocation::Stonecutter(_) => todo!(),
        }

        ItemSlot::Empty
    }

    /// Whether the given item could be placed in this menu.
    ///
    /// TODO: right now this always returns true
    pub fn may_place(&self, target_slot_index: usize, item: &ItemSlot) -> bool {
        true
    }

    fn move_item_to_slots(
        &self,
        item: &mut ItemSlotData,
        target_slots: &mut [ItemSlot],
        reverse: bool,
    ) {
        //
    }

    fn move_item_to_slot(&self, item_slot: &mut ItemSlot, target_slot: &mut ItemSlot) {
        let ItemSlot::Present(item) = item_slot else {
            return;
        };
        match target_slot {
            ItemSlot::Empty => {
                // the target slot is empty, so we can just move the item there
                if self.may_place(item) {
                    if item.count > 64 {
                        *target_slot = ItemSlot::Present(item.split(64));
                    } else {
                        *target_slot = ItemSlot::Present(item.split(item.count));
                    }
                }
            }
            ItemSlot::Present(_) => todo!(),
        }
    }
}
