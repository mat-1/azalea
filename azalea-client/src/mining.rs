use azalea_block::{Block, BlockState, FluidState};
use azalea_core::{direction::Direction, game_type::GameMode, position::BlockPos, tick::GameTick};
use azalea_entity::{mining::get_mine_progress, FluidOnEyes, Physics};
use azalea_inventory::ItemSlot;
use azalea_physics::PhysicsSet;
use azalea_protocol::packets::game::serverbound_player_action_packet::{
    self, ServerboundPlayerActionPacket,
};
use azalea_world::{InstanceContainer, InstanceName};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};

use crate::{
    interact::{
        can_use_game_master_blocks, check_is_interaction_restricted, CurrentSequenceNumber,
        HitResultComponent, SwingArmEvent,
    },
    inventory::{InventoryComponent, InventorySet},
    local_player::{LocalGameMode, PermissionLevel, PlayerAbilities},
    movement::MoveEventsSet,
    packet_handling::game::SendPacketEvent,
};

/// A plugin that allows clients to break blocks in the world.
pub struct MinePlugin;
impl Plugin for MinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartMiningBlockEvent>()
            .add_event::<StartMiningBlockWithDirectionEvent>()
            .add_event::<FinishMiningBlockEvent>()
            .add_event::<StopMiningBlockEvent>()
            .add_event::<MineBlockProgressEvent>()
            .add_event::<AttackBlockEvent>()
            .add_systems(GameTick, continue_mining_block.before(PhysicsSet))
            .add_systems(
                Update,
                (
                    handle_start_mining_block_event,
                    handle_start_mining_block_with_direction_event,
                    handle_finish_mining_block_event,
                    handle_stop_mining_block_event,
                )
                    .chain()
                    .in_set(MiningSet)
                    .after(InventorySet)
                    .after(MoveEventsSet)
                    .before(azalea_entity::update_bounding_box)
                    .after(azalea_entity::update_fluid_on_eyes)
                    .after(crate::interact::update_hit_result_component)
                    .after(crate::attack::handle_attack_event)
                    .after(crate::interact::handle_block_interact_event)
                    .before(crate::interact::handle_swing_arm_event),
            );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MiningSet;

/// Information about the block we're currently mining. This is only present if
/// we're currently mining a block.
#[derive(Component)]
pub struct Mining {
    pub pos: BlockPos,
    pub dir: Direction,
}

/// Start mining the block at the given position.
///
/// If we're looking at the block then the correct direction will be used,
/// otherwise it'll be [`Direction::Down`].
#[derive(Event)]
pub struct StartMiningBlockEvent {
    pub entity: Entity,
    pub position: BlockPos,
}
fn handle_start_mining_block_event(
    mut events: EventReader<StartMiningBlockEvent>,
    mut start_mining_events: EventWriter<StartMiningBlockWithDirectionEvent>,
    mut query: Query<&HitResultComponent>,
) {
    for event in events.read() {
        let hit_result = query.get_mut(event.entity).unwrap();
        let direction = if hit_result.block_pos == event.position {
            // we're looking at the block
            hit_result.direction
        } else {
            // we're not looking at the block, arbitrary direction
            Direction::Down
        };
        start_mining_events.send(StartMiningBlockWithDirectionEvent {
            entity: event.entity,
            position: event.position,
            direction,
        });
    }
}

#[derive(Event)]
pub struct StartMiningBlockWithDirectionEvent {
    pub entity: Entity,
    pub position: BlockPos,
    pub direction: Direction,
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn handle_start_mining_block_with_direction_event(
    mut events: EventReader<StartMiningBlockWithDirectionEvent>,
    mut finish_mining_events: EventWriter<FinishMiningBlockEvent>,
    mut send_packet_events: EventWriter<SendPacketEvent>,
    mut attack_block_events: EventWriter<AttackBlockEvent>,
    mut mine_block_progress_events: EventWriter<MineBlockProgressEvent>,
    mut query: Query<(
        &InstanceName,
        &LocalGameMode,
        &InventoryComponent,
        &FluidOnEyes,
        &Physics,
        Option<&Mining>,
        &mut CurrentSequenceNumber,
        &mut MineDelay,
        &mut MineProgress,
        &mut MineTicks,
        &mut MineItem,
        &mut MineBlockPos,
    )>,
    instances: Res<InstanceContainer>,
    mut commands: Commands,
) {
    for event in events.read() {
        let (
            instance_name,
            game_mode,
            inventory,
            fluid_on_eyes,
            physics,
            mining,
            mut sequence_number,
            mut mine_delay,
            mut mine_progress,
            mut mine_ticks,
            mut current_mining_item,
            mut current_mining_pos,
        ) = query.get_mut(event.entity).unwrap();

        let instance_lock = instances.get(instance_name).unwrap();
        let instance = instance_lock.read();
        if check_is_interaction_restricted(
            &instance,
            &event.position,
            &game_mode.current,
            inventory,
        ) {
            continue;
        }
        // TODO (when world border is implemented): vanilla ignores if the block
        // is outside of the worldborder

        if game_mode.current == GameMode::Creative {
            *sequence_number += 1;
            finish_mining_events.send(FinishMiningBlockEvent {
                entity: event.entity,
                position: event.position,
            });
            **mine_delay = 5;
        } else if mining.is_none()
            || !is_same_mining_target(
                event.position,
                inventory,
                &current_mining_pos,
                &current_mining_item,
            )
        {
            if mining.is_some() {
                // send a packet to stop mining since we just changed target
                send_packet_events.send(SendPacketEvent {
                    entity: event.entity,
                    packet: ServerboundPlayerActionPacket {
                        action: serverbound_player_action_packet::Action::AbortDestroyBlock,
                        pos: current_mining_pos
                            .expect("IsMining is true so MineBlockPos must be present"),
                        direction: event.direction,
                        sequence: 0,
                    }
                    .get(),
                });
            }

            let target_block_state = instance
                .get_block_state(&event.position)
                .unwrap_or_default();
            *sequence_number += 1;
            let target_registry_block = azalea_registry::Block::from(target_block_state);

            // we can't break blocks if they don't have a bounding box

            // TODO: So right now azalea doesn't differenciate between different types of
            // bounding boxes. See ClipContext::block_shape for more info. Ideally this
            // should just call ClipContext::block_shape and check if it's empty.
            let block_is_solid = !target_block_state.is_air()
                // this is a hack to make sure we can't break water or lava
                && !matches!(
                    target_registry_block,
                    azalea_registry::Block::Water | azalea_registry::Block::Lava
                );

            if block_is_solid && **mine_progress == 0. {
                // interact with the block (like note block left click) here
                attack_block_events.send(AttackBlockEvent {
                    entity: event.entity,
                    position: event.position,
                });
            }

            let block = Box::<dyn Block>::from(target_block_state);

            let held_item = inventory.held_item();

            if block_is_solid
                && get_mine_progress(
                    block.as_ref(),
                    held_item.kind(),
                    &inventory.inventory_menu,
                    fluid_on_eyes,
                    physics,
                ) >= 1.
            {
                // block was broken instantly
                finish_mining_events.send(FinishMiningBlockEvent {
                    entity: event.entity,
                    position: event.position,
                });
            } else {
                commands.entity(event.entity).insert(Mining {
                    pos: event.position,
                    dir: event.direction,
                });
                **current_mining_pos = Some(event.position);
                **current_mining_item = held_item;
                **mine_progress = 0.;
                **mine_ticks = 0.;
                mine_block_progress_events.send(MineBlockProgressEvent {
                    entity: event.entity,
                    position: event.position,
                    destroy_stage: mine_progress.destroy_stage(),
                });
            }

            send_packet_events.send(SendPacketEvent {
                entity: event.entity,
                packet: ServerboundPlayerActionPacket {
                    action: serverbound_player_action_packet::Action::StartDestroyBlock,
                    pos: event.position,
                    direction: event.direction,
                    sequence: **sequence_number,
                }
                .get(),
            });
        }
    }
}

#[derive(Event)]
pub struct MineBlockProgressEvent {
    pub entity: Entity,
    pub position: BlockPos,
    pub destroy_stage: Option<u32>,
}

/// A player left clicked on a block, used for stuff like interacting with note
/// blocks.
#[derive(Event)]
pub struct AttackBlockEvent {
    pub entity: Entity,
    pub position: BlockPos,
}

/// Returns whether the block and item are still the same as when we started
/// mining.
fn is_same_mining_target(
    target_block: BlockPos,
    inventory: &InventoryComponent,
    current_mining_pos: &MineBlockPos,
    current_mining_item: &MineItem,
) -> bool {
    let held_item = inventory.held_item();
    Some(target_block) == current_mining_pos.0 && held_item == current_mining_item.0
}

/// A component bundle for players that can mine blocks.
#[derive(Bundle, Default)]
pub struct MineBundle {
    pub delay: MineDelay,
    pub progress: MineProgress,
    pub ticks: MineTicks,
    pub mining_pos: MineBlockPos,
    pub mine_item: MineItem,
}

/// A component that counts down until we start mining the next block.
#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct MineDelay(pub u32);

/// A component that stores the progress of the current mining operation. This
/// is a value between 0 and 1.
#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct MineProgress(pub f32);

impl MineProgress {
    pub fn destroy_stage(&self) -> Option<u32> {
        if self.0 > 0. {
            Some((self.0 * 10.) as u32)
        } else {
            None
        }
    }
}

/// A component that stores the number of ticks that we've been mining the same
/// block for. This is a float even though it should only ever be a round
/// number.
#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct MineTicks(pub f32);

/// A component that stores the position of the block we're currently mining.
#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct MineBlockPos(pub Option<BlockPos>);

/// A component that contains the item we're currently using to mine. If we're
/// not mining anything, it'll be [`ItemSlot::Empty`].
#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct MineItem(pub ItemSlot);

/// Sent when we completed mining a block.
#[derive(Event)]
pub struct FinishMiningBlockEvent {
    pub entity: Entity,
    pub position: BlockPos,
}

fn handle_finish_mining_block_event(
    mut events: EventReader<FinishMiningBlockEvent>,
    mut query: Query<(
        &InstanceName,
        &LocalGameMode,
        &InventoryComponent,
        &PlayerAbilities,
        &PermissionLevel,
        &mut CurrentSequenceNumber,
    )>,
    instances: Res<InstanceContainer>,
) {
    for event in events.read() {
        let (instance_name, game_mode, inventory, abilities, permission_level, _sequence_number) =
            query.get_mut(event.entity).unwrap();
        let instance_lock = instances.get(instance_name).unwrap();
        let instance = instance_lock.read();
        if check_is_interaction_restricted(
            &instance,
            &event.position,
            &game_mode.current,
            inventory,
        ) {
            continue;
        }

        if game_mode.current == GameMode::Creative {
            let held_item = inventory.held_item().kind();
            if matches!(
                held_item,
                azalea_registry::Item::Trident | azalea_registry::Item::DebugStick
            ) || azalea_registry::tags::items::SWORDS.contains(&held_item)
            {
                continue;
            }
        }

        let Some(block_state) = instance.get_block_state(&event.position) else {
            continue;
        };

        let registry_block = Box::<dyn Block>::from(block_state).as_registry_block();
        if !can_use_game_master_blocks(abilities, permission_level)
            && matches!(
                registry_block,
                azalea_registry::Block::CommandBlock | azalea_registry::Block::StructureBlock
            )
        {
            continue;
        }
        if block_state == BlockState::AIR {
            continue;
        }

        // when we break a waterlogged block we want to keep the water there
        let fluid_state = FluidState::from(block_state);
        let block_state_for_fluid = BlockState::from(fluid_state);
        instance.set_block_state(&event.position, block_state_for_fluid);
    }
}

/// Abort mining a block.
#[derive(Event)]
pub struct StopMiningBlockEvent {
    pub entity: Entity,
}
fn handle_stop_mining_block_event(
    mut events: EventReader<StopMiningBlockEvent>,
    mut send_packet_events: EventWriter<SendPacketEvent>,
    mut mine_block_progress_events: EventWriter<MineBlockProgressEvent>,
    mut query: Query<(&mut Mining, &MineBlockPos, &mut MineProgress)>,
    mut commands: Commands,
) {
    for event in events.read() {
        let (mut _mining, mine_block_pos, mut mine_progress) = query.get_mut(event.entity).unwrap();

        let mine_block_pos =
            mine_block_pos.expect("IsMining is true so MineBlockPos must be present");
        send_packet_events.send(SendPacketEvent {
            entity: event.entity,
            packet: ServerboundPlayerActionPacket {
                action: serverbound_player_action_packet::Action::AbortDestroyBlock,
                pos: mine_block_pos,
                direction: Direction::Down,
                sequence: 0,
            }
            .get(),
        });
        commands.entity(event.entity).remove::<Mining>();
        **mine_progress = 0.;
        mine_block_progress_events.send(MineBlockProgressEvent {
            entity: event.entity,
            position: mine_block_pos,
            destroy_stage: None,
        });
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn continue_mining_block(
    mut query: Query<(
        Entity,
        &InstanceName,
        &LocalGameMode,
        &InventoryComponent,
        &MineBlockPos,
        &MineItem,
        &FluidOnEyes,
        &Physics,
        &Mining,
        &mut MineDelay,
        &mut MineProgress,
        &mut MineTicks,
        &mut CurrentSequenceNumber,
    )>,
    mut send_packet_events: EventWriter<SendPacketEvent>,
    mut mine_block_progress_events: EventWriter<MineBlockProgressEvent>,
    mut finish_mining_events: EventWriter<FinishMiningBlockEvent>,
    mut start_mining_events: EventWriter<StartMiningBlockWithDirectionEvent>,
    mut swing_arm_events: EventWriter<SwingArmEvent>,
    instances: Res<InstanceContainer>,
    mut commands: Commands,
) {
    for (
        entity,
        instance_name,
        game_mode,
        inventory,
        current_mining_pos,
        current_mining_item,
        fluid_on_eyes,
        physics,
        mining,
        mut mine_delay,
        mut mine_progress,
        mut mine_ticks,
        mut sequence_number,
    ) in query.iter_mut()
    {
        if **mine_delay > 0 {
            **mine_delay -= 1;
            continue;
        }

        if game_mode.current == GameMode::Creative {
            // TODO: worldborder check
            **mine_delay = 5;
            finish_mining_events.send(FinishMiningBlockEvent {
                entity,
                position: mining.pos,
            });
            *sequence_number += 1;
            send_packet_events.send(SendPacketEvent {
                entity,
                packet: ServerboundPlayerActionPacket {
                    action: serverbound_player_action_packet::Action::StartDestroyBlock,
                    pos: mining.pos,
                    direction: mining.dir,
                    sequence: **sequence_number,
                }
                .get(),
            });
            swing_arm_events.send(SwingArmEvent { entity });
        } else if is_same_mining_target(
            mining.pos,
            inventory,
            current_mining_pos,
            current_mining_item,
        ) {
            let instance_lock = instances.get(instance_name).unwrap();
            let instance = instance_lock.read();
            let target_block_state = instance.get_block_state(&mining.pos).unwrap_or_default();

            if target_block_state.is_air() {
                commands.entity(entity).remove::<Mining>();
                continue;
            }
            let block = Box::<dyn Block>::from(target_block_state);
            **mine_progress += get_mine_progress(
                block.as_ref(),
                current_mining_item.kind(),
                &inventory.inventory_menu,
                fluid_on_eyes,
                physics,
            );

            if **mine_ticks % 4. == 0. {
                // vanilla makes a mining sound here
            }
            **mine_ticks += 1.;

            if **mine_progress >= 1. {
                commands.entity(entity).remove::<Mining>();
                *sequence_number += 1;
                finish_mining_events.send(FinishMiningBlockEvent {
                    entity,
                    position: mining.pos,
                });
                send_packet_events.send(SendPacketEvent {
                    entity,
                    packet: ServerboundPlayerActionPacket {
                        action: serverbound_player_action_packet::Action::StopDestroyBlock,
                        pos: mining.pos,
                        direction: mining.dir,
                        sequence: **sequence_number,
                    }
                    .get(),
                });
                **mine_progress = 0.;
                **mine_ticks = 0.;
                **mine_delay = 0;
            }

            mine_block_progress_events.send(MineBlockProgressEvent {
                entity,
                position: mining.pos,
                destroy_stage: mine_progress.destroy_stage(),
            });
            swing_arm_events.send(SwingArmEvent { entity });
        } else {
            start_mining_events.send(StartMiningBlockWithDirectionEvent {
                entity,
                position: mining.pos,
                direction: mining.dir,
            });
        }

        swing_arm_events.send(SwingArmEvent { entity });
    }
}
