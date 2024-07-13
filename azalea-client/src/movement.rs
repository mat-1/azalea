use crate::packet_handling::game::SendPacketEvent;
use azalea_core::position::Vec3;
use azalea_core::tick::GameTick;
use azalea_entity::{metadata::Sprinting, Attributes, Jumping};
use azalea_entity::{InLoadedChunk, LastSentPosition, LookDirection, Physics, Position};
use azalea_physics::{ai_step, PhysicsSet};
use azalea_protocol::packets::game::serverbound_player_command_packet::ServerboundPlayerCommandPacket;
use azalea_protocol::packets::game::{
    serverbound_move_player_pos_packet::ServerboundMovePlayerPosPacket,
    serverbound_move_player_pos_rot_packet::ServerboundMovePlayerPosRotPacket,
    serverbound_move_player_rot_packet::ServerboundMovePlayerRotPacket,
    serverbound_move_player_status_only_packet::ServerboundMovePlayerStatusOnlyPacket,
};
use azalea_world::{MinecraftEntityId, MoveEntityError};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::{Event, EventWriter};
use bevy_ecs::schedule::SystemSet;
use bevy_ecs::{
    component::Component, entity::Entity, event::EventReader, query::With,
    schedule::IntoSystemConfigs, system::Query,
};
use std::backtrace::Backtrace;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MovePlayerError {
    #[error("Player is not in world")]
    PlayerNotInWorld(Backtrace),
    #[error("{0}")]
    Io(#[from] std::io::Error),
}

impl From<MoveEntityError> for MovePlayerError {
    fn from(err: MoveEntityError) -> Self {
        match err {
            MoveEntityError::EntityDoesNotExist(backtrace) => {
                MovePlayerError::PlayerNotInWorld(backtrace)
            }
        }
    }
}

pub struct PlayerMovePlugin;

impl Plugin for PlayerMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartWalkEvent>()
            .add_event::<StartSprintEvent>()
            .add_event::<KnockbackEvent>()
            .add_systems(
                Update,
                (handle_sprint, handle_walk, handle_knockback)
                    .chain()
                    .in_set(MoveEventsSet),
            )
            .add_systems(
                GameTick,
                (
                    (tick_controls, local_player_ai_step)
                        .chain()
                        .in_set(PhysicsSet)
                        .before(ai_step),
                    send_sprinting_if_needed.after(azalea_entity::update_in_loaded_chunk),
                    send_position.after(PhysicsSet),
                )
                    .chain(),
            );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MoveEventsSet;

/// A component that contains the look direction that was last sent over the
/// network.
#[derive(Debug, Component, Clone, Default)]
pub struct LastSentLookDirection {
    pub x_rot: f32,
    pub y_rot: f32,
}

/// Component for entities that can move and sprint. Usually only in
/// [`LocalEntity`]s.
///
/// [`LocalEntity`]: azalea_entity::LocalEntity
#[derive(Default, Component, Clone)]
pub struct PhysicsState {
    /// Minecraft only sends a movement packet either after 20 ticks or if the
    /// player moved enough. This is that tick counter.
    pub position_remainder: u32,
    pub was_sprinting: bool,
    // Whether we're going to try to start sprinting this tick. Equivalent to
    // holding down ctrl for a tick.
    pub trying_to_sprint: bool,

    pub move_direction: WalkDirection,
    pub forward_impulse: f32,
    pub left_impulse: f32,
}

#[allow(clippy::type_complexity)]
pub fn send_position(
    mut query: Query<
        (
            Entity,
            &Position,
            &LookDirection,
            &mut PhysicsState,
            &mut LastSentPosition,
            &mut Physics,
            &mut LastSentLookDirection,
        ),
        With<InLoadedChunk>,
    >,
    mut send_packet_events: EventWriter<SendPacketEvent>,
) {
    for (
        entity,
        position,
        direction,
        mut physics_state,
        mut last_sent_position,
        mut physics,
        mut last_direction,
    ) in query.iter_mut()
    {
        let packet = {
            // TODO: the camera being able to be controlled by other entities isn't
            // implemented yet if !self.is_controlled_camera() { return };

            let x_delta = position.x - last_sent_position.x;
            let y_delta = position.y - last_sent_position.y;
            let z_delta = position.z - last_sent_position.z;
            let y_rot_delta = (direction.y_rot - last_direction.y_rot) as f64;
            let x_rot_delta = (direction.x_rot - last_direction.x_rot) as f64;

            physics_state.position_remainder += 1;

            // boolean sendingPosition = Mth.lengthSquared(xDelta, yDelta, zDelta) >
            // Mth.square(2.0E-4D) || this.positionReminder >= 20;
            let sending_position = ((x_delta.powi(2) + y_delta.powi(2) + z_delta.powi(2))
                > 2.0e-4f64.powi(2))
                || physics_state.position_remainder >= 20;
            let sending_direction = y_rot_delta != 0.0 || x_rot_delta != 0.0;

            // if self.is_passenger() {
            //   TODO: posrot packet for being a passenger
            // }
            let packet = if sending_position && sending_direction {
                Some(
                    ServerboundMovePlayerPosRotPacket {
                        x: position.x,
                        y: position.y,
                        z: position.z,
                        x_rot: direction.x_rot,
                        y_rot: direction.y_rot,
                        on_ground: physics.on_ground,
                    }
                    .get(),
                )
            } else if sending_position {
                Some(
                    ServerboundMovePlayerPosPacket {
                        x: position.x,
                        y: position.y,
                        z: position.z,
                        on_ground: physics.on_ground,
                    }
                    .get(),
                )
            } else if sending_direction {
                Some(
                    ServerboundMovePlayerRotPacket {
                        x_rot: direction.x_rot,
                        y_rot: direction.y_rot,
                        on_ground: physics.on_ground,
                    }
                    .get(),
                )
            } else if physics.last_on_ground != physics.on_ground {
                Some(
                    ServerboundMovePlayerStatusOnlyPacket {
                        on_ground: physics.on_ground,
                    }
                    .get(),
                )
            } else {
                None
            };

            if sending_position {
                **last_sent_position = **position;
                physics_state.position_remainder = 0;
            }
            if sending_direction {
                last_direction.y_rot = direction.y_rot;
                last_direction.x_rot = direction.x_rot;
            }

            physics.last_on_ground = physics.on_ground;
            // minecraft checks for autojump here, but also autojump is bad so

            packet
        };

        if let Some(packet) = packet {
            send_packet_events.send(SendPacketEvent { entity, packet });
        }
    }
}

fn send_sprinting_if_needed(
    mut query: Query<(Entity, &MinecraftEntityId, &Sprinting, &mut PhysicsState)>,
    mut send_packet_events: EventWriter<SendPacketEvent>,
) {
    for (entity, minecraft_entity_id, sprinting, mut physics_state) in query.iter_mut() {
        let was_sprinting = physics_state.was_sprinting;
        if **sprinting != was_sprinting {
            let sprinting_action = if **sprinting {
                azalea_protocol::packets::game::serverbound_player_command_packet::Action::StartSprinting
            } else {
                azalea_protocol::packets::game::serverbound_player_command_packet::Action::StopSprinting
            };
            send_packet_events.send(SendPacketEvent {
                entity,
                packet: ServerboundPlayerCommandPacket {
                    id: **minecraft_entity_id,
                    action: sprinting_action,
                    data: 0,
                }
                .get(),
            });
            physics_state.was_sprinting = **sprinting;
        }
    }
}

/// Update the impulse from self.move_direction. The multiplier is used for
/// sneaking.
pub(crate) fn tick_controls(mut query: Query<&mut PhysicsState>) {
    for mut physics_state in query.iter_mut() {
        let multiplier: Option<f32> = None;

        let mut forward_impulse: f32 = 0.;
        let mut left_impulse: f32 = 0.;
        let move_direction = physics_state.move_direction;
        match move_direction {
            WalkDirection::Forward | WalkDirection::ForwardRight | WalkDirection::ForwardLeft => {
                forward_impulse += 1.;
            }
            WalkDirection::Backward
            | WalkDirection::BackwardRight
            | WalkDirection::BackwardLeft => {
                forward_impulse -= 1.;
            }
            _ => {}
        };
        match move_direction {
            WalkDirection::Right | WalkDirection::ForwardRight | WalkDirection::BackwardRight => {
                left_impulse += 1.;
            }
            WalkDirection::Left | WalkDirection::ForwardLeft | WalkDirection::BackwardLeft => {
                left_impulse -= 1.;
            }
            _ => {}
        };
        physics_state.forward_impulse = forward_impulse;
        physics_state.left_impulse = left_impulse;

        if let Some(multiplier) = multiplier {
            physics_state.forward_impulse *= multiplier;
            physics_state.left_impulse *= multiplier;
        }
    }
}

/// Makes the bot do one physics tick. Note that this is already handled
/// automatically by the client.
pub fn local_player_ai_step(
    mut query: Query<
        (&PhysicsState, &mut Physics, &mut Sprinting, &mut Attributes),
        With<InLoadedChunk>,
    >,
) {
    for (physics_state, mut physics, mut sprinting, mut attributes) in query.iter_mut() {
        // server ai step
        physics.xxa = physics_state.left_impulse;
        physics.zza = physics_state.forward_impulse;

        // TODO: food data and abilities
        // let has_enough_food_to_sprint = self.food_data().food_level ||
        // self.abilities().may_fly;
        let has_enough_food_to_sprint = true;

        // TODO: double tapping w to sprint i think

        let trying_to_sprint = physics_state.trying_to_sprint;

        if !**sprinting
            && (
                // !self.is_in_water()
                // || self.is_underwater() &&
                has_enough_impulse_to_start_sprinting(physics_state)
                    && has_enough_food_to_sprint
                    // && !self.using_item()
                    // && !self.has_effect(MobEffects.BLINDNESS)
                    && trying_to_sprint
            )
        {
            set_sprinting(true, &mut sprinting, &mut attributes);
        }
    }
}

/// An event sent when the client starts walking. This does not get sent for
/// non-local entities.
///
/// To stop walking or sprinting, send this event with `WalkDirection::None`.
#[derive(Event, Debug)]
pub struct StartWalkEvent {
    pub entity: Entity,
    pub direction: WalkDirection,
}

/// The system that makes the player start walking when they receive a
/// [`StartWalkEvent`].
pub fn handle_walk(
    mut events: EventReader<StartWalkEvent>,
    mut query: Query<(&mut PhysicsState, &mut Sprinting, &mut Attributes)>,
) {
    for event in events.read() {
        if let Ok((mut physics_state, mut sprinting, mut attributes)) = query.get_mut(event.entity)
        {
            physics_state.move_direction = event.direction;
            physics_state.trying_to_sprint = false;
            set_sprinting(false, &mut sprinting, &mut attributes);
        }
    }
}

/// An event sent when the client starts sprinting. This does not get sent for
/// non-local entities.
#[derive(Event)]
pub struct StartSprintEvent {
    pub entity: Entity,
    pub direction: SprintDirection,
}
/// The system that makes the player start sprinting when they receive a
/// [`StartSprintEvent`].
pub fn handle_sprint(
    mut query: Query<&mut PhysicsState>,
    mut events: EventReader<StartSprintEvent>,
) {
    for event in events.read() {
        if let Ok(mut physics_state) = query.get_mut(event.entity) {
            physics_state.move_direction = WalkDirection::from(event.direction);
            physics_state.trying_to_sprint = true;
        }
    }
}

/// Change whether we're sprinting by adding an attribute modifier to the
/// player. You should use the [`walk`] and [`sprint`] methods instead.
/// Returns if the operation was successful.
fn set_sprinting(
    sprinting: bool,
    currently_sprinting: &mut Sprinting,
    attributes: &mut Attributes,
) -> bool {
    **currently_sprinting = sprinting;
    if sprinting {
        attributes
            .speed
            .insert(azalea_entity::attributes::sprinting_modifier())
            .is_ok()
    } else {
        attributes
            .speed
            .remove(&azalea_entity::attributes::sprinting_modifier().id)
            .is_none()
    }
}

// Whether the player is moving fast enough to be able to start sprinting.
fn has_enough_impulse_to_start_sprinting(physics_state: &PhysicsState) -> bool {
    // if self.underwater() {
    //     self.has_forward_impulse()
    // } else {
    physics_state.forward_impulse > 0.8
    // }
}

/// An event sent by the server that sets or adds to our velocity. Usually
/// `KnockbackKind::Set` is used for normal knockback and `KnockbackKind::Add`
/// is used for explosions, but some servers (notably Hypixel) use explosions
/// for knockback.
#[derive(Event)]
pub struct KnockbackEvent {
    pub entity: Entity,
    pub knockback: KnockbackType,
}

pub enum KnockbackType {
    Set(Vec3),
    Add(Vec3),
}

pub fn handle_knockback(mut query: Query<&mut Physics>, mut events: EventReader<KnockbackEvent>) {
    for event in events.read() {
        if let Ok(mut physics) = query.get_mut(event.entity) {
            match event.knockback {
                KnockbackType::Set(velocity) => {
                    physics.velocity = velocity;
                }
                KnockbackType::Add(velocity) => {
                    physics.velocity += velocity;
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum WalkDirection {
    #[default]
    None,
    Forward,
    Backward,
    Left,
    Right,
    ForwardRight,
    ForwardLeft,
    BackwardRight,
    BackwardLeft,
}

/// The directions that we can sprint in. It's a subset of [`WalkDirection`].
#[derive(Clone, Copy, Debug)]
pub enum SprintDirection {
    Forward,
    ForwardRight,
    ForwardLeft,
}

impl From<SprintDirection> for WalkDirection {
    fn from(d: SprintDirection) -> Self {
        match d {
            SprintDirection::Forward => WalkDirection::Forward,
            SprintDirection::ForwardRight => WalkDirection::ForwardRight,
            SprintDirection::ForwardLeft => WalkDirection::ForwardLeft,
        }
    }
}
