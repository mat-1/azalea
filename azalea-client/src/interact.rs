use std::ops::AddAssign;

use azalea_block::BlockState;
use azalea_core::{
    block_hit_result::BlockHitResult,
    direction::Direction,
    game_type::GameMode,
    position::{BlockPos, Vec3},
};
use azalea_entity::{
    clamp_look_direction, view_vector, Attributes, EyeHeight, LocalEntity, LookDirection, Position,
};
use azalea_inventory::{ItemSlot, ItemSlotData};
use azalea_physics::clip::{BlockShapeType, ClipContext, FluidPickType};
use azalea_protocol::packets::game::{
    serverbound_interact_packet::InteractionHand,
    serverbound_swing_packet::ServerboundSwingPacket,
    serverbound_use_item_on_packet::{BlockHit, ServerboundUseItemOnPacket},
};
use azalea_registry::DataComponentKind;
use azalea_world::{Instance, InstanceContainer, InstanceName};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::{Event, EventReader, EventWriter},
    query::{Changed, With},
    schedule::IntoSystemConfigs,
    system::{Commands, Query, Res},
};
use derive_more::{Deref, DerefMut};
use tracing::warn;

use crate::{
    attack::handle_attack_event,
    inventory::{InventoryComponent, InventorySet},
    local_player::{LocalGameMode, PermissionLevel, PlayerAbilities},
    movement::MoveEventsSet,
    packet_handling::game::{handle_send_packet_event, SendPacketEvent},
    respawn::perform_respawn,
    Client,
};

/// A plugin that allows clients to interact with blocks in the world.
pub struct InteractPlugin;
impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockInteractEvent>()
            .add_event::<SwingArmEvent>()
            .add_systems(
                Update,
                (
                    (
                        update_hit_result_component.after(clamp_look_direction),
                        handle_block_interact_event,
                        handle_swing_arm_event,
                    )
                        .before(handle_send_packet_event)
                        .after(InventorySet)
                        .after(perform_respawn)
                        .after(handle_attack_event)
                        .chain(),
                    update_modifiers_for_held_item
                        .after(InventorySet)
                        .after(MoveEventsSet),
                ),
            );
    }
}

impl Client {
    /// Right click a block. The behavior of this depends on the target block,
    /// and it'll either place the block you're holding in your hand or use the
    /// block you clicked (like toggling a lever).
    ///
    /// Note that this may trigger anticheats as it doesn't take into account
    /// whether you're actually looking at the block.
    pub fn block_interact(&mut self, position: BlockPos) {
        self.ecs.lock().send_event(BlockInteractEvent {
            entity: self.entity,
            position,
        });
    }
}

/// Right click a block. The behavior of this depends on the target block,
/// and it'll either place the block you're holding in your hand or use the
/// block you clicked (like toggling a lever).
#[derive(Event)]
pub struct BlockInteractEvent {
    /// The local player entity that's opening the container.
    pub entity: Entity,
    /// The coordinates of the container.
    pub position: BlockPos,
}

/// A component that contains the number of changes this client has made to
/// blocks.
#[derive(Component, Copy, Clone, Debug, Default, Deref)]
pub struct CurrentSequenceNumber(u32);

impl AddAssign<u32> for CurrentSequenceNumber {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs;
    }
}

/// A component that contains the block that the player is currently looking at.
#[derive(Component, Clone, Debug, Deref, DerefMut)]
pub struct HitResultComponent(BlockHitResult);

pub fn handle_block_interact_event(
    mut events: EventReader<BlockInteractEvent>,
    mut query: Query<(Entity, &mut CurrentSequenceNumber, &HitResultComponent)>,
    mut send_packet_events: EventWriter<SendPacketEvent>,
) {
    for event in events.read() {
        let Ok((entity, mut sequence_number, hit_result)) = query.get_mut(event.entity) else {
            warn!("Sent BlockInteractEvent for entity that doesn't have the required components");
            continue;
        };

        // TODO: check to make sure we're within the world border

        *sequence_number += 1;

        // minecraft also does the interaction client-side (so it looks like clicking a
        // button is instant) but we don't really need that

        // the block_hit data will depend on whether we're looking at the block and
        // whether we can reach it

        let block_hit = if hit_result.block_pos == event.position {
            // we're looking at the block :)
            BlockHit {
                block_pos: hit_result.block_pos,
                direction: hit_result.direction,
                location: hit_result.location,
                inside: hit_result.inside,
            }
        } else {
            // we're not looking at the block, so make up some numbers
            BlockHit {
                block_pos: event.position,
                direction: Direction::Up,
                location: event.position.center(),
                inside: false,
            }
        };

        send_packet_events.send(SendPacketEvent {
            entity,
            packet: ServerboundUseItemOnPacket {
                hand: InteractionHand::MainHand,
                block_hit,
                sequence: sequence_number.0,
            }
            .get(),
        });
    }
}

#[allow(clippy::type_complexity)]
pub fn update_hit_result_component(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        Option<&mut HitResultComponent>,
        &LocalGameMode,
        &Position,
        &EyeHeight,
        &LookDirection,
        &InstanceName,
    )>,
    instance_container: Res<InstanceContainer>,
) {
    for (entity, hit_result_ref, game_mode, position, eye_height, look_direction, world_name) in
        &mut query
    {
        let pick_range = if game_mode.current == GameMode::Creative {
            6.
        } else {
            4.5
        };
        let eye_position = Vec3 {
            x: position.x,
            y: position.y + **eye_height as f64,
            z: position.z,
        };

        let Some(instance_lock) = instance_container.get(world_name) else {
            continue;
        };
        let instance = instance_lock.read();

        let hit_result = pick(look_direction, &eye_position, &instance.chunks, pick_range);
        if let Some(mut hit_result_ref) = hit_result_ref {
            **hit_result_ref = hit_result;
        } else {
            commands
                .entity(entity)
                .insert(HitResultComponent(hit_result));
        }
    }
}

/// Get the block that a player would be looking at if their eyes were at the
/// given direction and position.
///
/// If you need to get the block the player is looking at right now, use
/// [`HitResultComponent`].
pub fn pick(
    look_direction: &LookDirection,
    eye_position: &Vec3,
    chunks: &azalea_world::ChunkStorage,
    pick_range: f64,
) -> BlockHitResult {
    let view_vector = view_vector(look_direction);
    let end_position = eye_position + &(view_vector * pick_range);
    azalea_physics::clip::clip(
        chunks,
        ClipContext {
            from: *eye_position,
            to: end_position,
            block_shape_type: BlockShapeType::Outline,
            fluid_pick_type: FluidPickType::None,
        },
    )
}

/// Whether we can't interact with the block, based on your gamemode. If
/// this is false, then we can interact with the block.
///
/// Passing the inventory, block position, and instance is necessary for the
/// adventure mode check.
pub fn check_is_interaction_restricted(
    instance: &Instance,
    block_pos: &BlockPos,
    game_mode: &GameMode,
    inventory: &InventoryComponent,
) -> bool {
    match game_mode {
        GameMode::Adventure => {
            // vanilla checks for abilities.mayBuild here but servers have no
            // way of modifying that

            let held_item = inventory.held_item();
            if let ItemSlot::Present(item) = &held_item {
                let block = instance.chunks.get_block_state(block_pos);
                let Some(block) = block else {
                    // block isn't loaded so just say that it is restricted
                    return true;
                };
                check_block_can_be_broken_by_item_in_adventure_mode(item, &block)
            } else {
                true
            }
        }
        GameMode::Spectator => true,
        _ => false,
    }
}

/// Check if the item has the `CanDestroy` tag for the block.
pub fn check_block_can_be_broken_by_item_in_adventure_mode(
    item: &ItemSlotData,
    _block: &BlockState,
) -> bool {
    // minecraft caches the last checked block but that's kind of an unnecessary
    // optimization and makes the code too complicated

    let Some(_can_destroy) = item.components.get(DataComponentKind::CanBreak) else {
        // no CanDestroy tag
        return false;
    };

    false

    // for block_predicate in can_destroy {
    //     // TODO
    //     // defined in BlockPredicateArgument.java
    // }

    // true
}

pub fn can_use_game_master_blocks(
    abilities: &PlayerAbilities,
    permission_level: &PermissionLevel,
) -> bool {
    abilities.instant_break && **permission_level >= 2
}

/// Swing your arm. This is purely a visual effect and won't interact with
/// anything in the world.
#[derive(Event)]
pub struct SwingArmEvent {
    pub entity: Entity,
}
pub fn handle_swing_arm_event(
    mut events: EventReader<SwingArmEvent>,
    mut send_packet_events: EventWriter<SendPacketEvent>,
) {
    for event in events.read() {
        send_packet_events.send(SendPacketEvent {
            entity: event.entity,
            packet: ServerboundSwingPacket {
                hand: InteractionHand::MainHand,
            }
            .get(),
        });
    }
}

#[allow(clippy::type_complexity)]
fn update_modifiers_for_held_item(
    mut query: Query<
        (&mut Attributes, &InventoryComponent),
        (With<LocalEntity>, Changed<InventoryComponent>),
    >,
) {
    for (mut attributes, inventory) in &mut query {
        let held_item = inventory.held_item();

        use azalea_registry::Item;
        let added_attack_speed = match held_item.kind() {
            Item::WoodenSword => -2.4,
            Item::WoodenShovel => -3.0,
            Item::WoodenPickaxe => -2.8,
            Item::WoodenAxe => -3.2,
            Item::WoodenHoe => -3.0,

            Item::StoneSword => -2.4,
            Item::StoneShovel => -3.0,
            Item::StonePickaxe => -2.8,
            Item::StoneAxe => -3.2,
            Item::StoneHoe => -2.0,

            Item::GoldenSword => -2.4,
            Item::GoldenShovel => -3.0,
            Item::GoldenPickaxe => -2.8,
            Item::GoldenAxe => -3.0,
            Item::GoldenHoe => -3.0,

            Item::IronSword => -2.4,
            Item::IronShovel => -3.0,
            Item::IronPickaxe => -2.8,
            Item::IronAxe => -3.1,
            Item::IronHoe => -1.0,

            Item::DiamondSword => -2.4,
            Item::DiamondShovel => -3.0,
            Item::DiamondPickaxe => -2.8,
            Item::DiamondAxe => -3.0,
            Item::DiamondHoe => 0.0,

            Item::NetheriteSword => -2.4,
            Item::NetheriteShovel => -3.0,
            Item::NetheritePickaxe => -2.8,
            Item::NetheriteAxe => -3.0,
            Item::NetheriteHoe => 0.0,

            Item::Trident => -2.9,
            _ => 0.,
        };
        attributes
            .attack_speed
            .insert(azalea_entity::attributes::base_attack_speed_modifier(
                added_attack_speed,
            ));
    }
}

#[cfg(test)]
mod tests {
    use azalea_core::position::ChunkPos;
    use azalea_world::{Chunk, ChunkStorage, PartialInstance};

    use super::*;

    #[test]
    fn test_pick() {
        let mut partial_world = PartialInstance::default();
        let mut world = ChunkStorage::default();

        partial_world
            .chunks
            .update_view_center(ChunkPos { x: -184, z: -2 });
        partial_world.chunks.set(
            &ChunkPos { x: -184, z: -2 },
            Some(Chunk::default()),
            &mut world,
        );

        let set_block = |x, y, z| {
            partial_world
                .chunks
                .set_block_state(
                    &BlockPos::new(x, y, z),
                    azalea_registry::Block::Stone.into(),
                    &world,
                )
                .expect(&format!("failed to set block at {x} {y} {z}"));
        };

        for x in -2940..=-2936 {
            for z in -24..=-20 {
                set_block(x, 64, z);
                set_block(x, 65, z);
            }
        }

        let hit_result = pick(
            &LookDirection {
                y_rot: 45.,
                x_rot: 35.66751,
                // x_rot: 35.,
            },
            &Vec3::new(-2936.5, 66. + 1.53, -22.5),
            &world,
            4.5,
        );

        assert!(!hit_result.miss);
        assert_eq!(hit_result.block_pos, BlockPos::new(-2939, 65, -21));
    }
}
