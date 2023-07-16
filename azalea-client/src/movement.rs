use crate::client::Client;
use crate::local_player::{update_in_loaded_chunk, LocalPlayer, LocalPlayerInLoadedChunk};
use azalea_entity::{metadata::Sprinting, Attributes, Jumping};
use azalea_entity::{LastSentPosition, LookDirection, Physics, Position};
use azalea_physics::{force_jump_listener, PhysicsSet};
use azalea_protocol::packets::game::serverbound_player_command_packet::ServerboundPlayerCommandPacket;
use azalea_protocol::packets::game::{
    serverbound_move_player_pos_packet::ServerboundMovePlayerPosPacket,
    serverbound_move_player_pos_rot_packet::ServerboundMovePlayerPosRotPacket,
    serverbound_move_player_rot_packet::ServerboundMovePlayerRotPacket,
    serverbound_move_player_status_only_packet::ServerboundMovePlayerStatusOnlyPacket,
};
use azalea_world::{MinecraftEntityId, MoveEntityError};
use bevy_app::{App, FixedUpdate, Plugin, Update};
use bevy_ecs::prelude::Event;
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
            .add_systems(
                Update,
                (sprint_listener, walk_listener)
                    .chain()
                    .before(force_jump_listener),
            )
            .add_systems(
                FixedUpdate,
                (
                    local_player_ai_step.in_set(PhysicsSet),
                    send_position.after(update_in_loaded_chunk),
                )
                    .chain(),
            );
    }
}

impl Client {
    /// Set whether we're jumping. This acts as if you held space in
    /// vanilla. If you want to jump once, use the `jump` function.
    ///
    /// If you're making a realistic client, calling this function every tick is
    /// recommended.
    pub fn set_jumping(&mut self, jumping: bool) {
        let mut ecs = self.ecs.lock();
        let mut jumping_mut = self.query::<&mut Jumping>(&mut ecs);
        **jumping_mut = jumping;
    }

    /// Returns whether the player will try to jump next tick.
    pub fn jumping(&self) -> bool {
        let mut ecs = self.ecs.lock();
        let jumping_ref = self.query::<&Jumping>(&mut ecs);
        **jumping_ref
    }

    /// Sets the direction the client is looking. `y_rot` is yaw (looking to the
    /// side), `x_rot` is pitch (looking up and down). You can get these
    /// numbers from the vanilla f3 screen.
    /// `y_rot` goes from -180 to 180, and `x_rot` goes from -90 to 90.
    pub fn set_direction(&mut self, y_rot: f32, x_rot: f32) {
        let mut ecs = self.ecs.lock();
        let mut look_direction = self.query::<&mut LookDirection>(&mut ecs);

        (look_direction.y_rot, look_direction.x_rot) = (y_rot, x_rot);
    }
}

/// Component for entities that can move and sprint. Usually only in
/// [`LocalPlayer`] entities.
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

/// A component that contains the look direction that was last sent over the
/// network.
#[derive(Debug, Component, Clone, Default)]
pub struct LastSentLookDirection {
    pub x_rot: f32,
    pub y_rot: f32,
}

#[allow(clippy::type_complexity)]
pub(crate) fn send_position(
    mut query: Query<
        (
            &MinecraftEntityId,
            &mut LocalPlayer,
            &mut PhysicsState,
            &Position,
            &mut LastSentPosition,
            &mut Physics,
            &LookDirection,
            &mut LastSentLookDirection,
            &Sprinting,
        ),
        &LocalPlayerInLoadedChunk,
    >,
) {
    for (
        id,
        mut local_player,
        mut physics_state,
        position,
        mut last_sent_position,
        mut physics,
        direction,
        mut last_direction,
        sprinting,
    ) in query.iter_mut()
    {
        local_player.send_sprinting_if_needed(id, sprinting, &mut physics_state);

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
            local_player.write_packet(packet);
        }
    }
}

impl LocalPlayer {
    fn send_sprinting_if_needed(
        &mut self,
        id: &MinecraftEntityId,
        sprinting: &Sprinting,
        physics_state: &mut PhysicsState,
    ) {
        let was_sprinting = physics_state.was_sprinting;
        if **sprinting != was_sprinting {
            let sprinting_action = if **sprinting {
                azalea_protocol::packets::game::serverbound_player_command_packet::Action::StartSprinting
            } else {
                azalea_protocol::packets::game::serverbound_player_command_packet::Action::StopSprinting
            };
            self.write_packet(
                ServerboundPlayerCommandPacket {
                    id: **id,
                    action: sprinting_action,
                    data: 0,
                }
                .get(),
            );
            physics_state.was_sprinting = **sprinting;
        }
    }

    /// Update the impulse from self.move_direction. The multipler is used for
    /// sneaking.
    pub(crate) fn tick_controls(multiplier: Option<f32>, physics_state: &mut PhysicsState) {
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
        (
            &mut PhysicsState,
            &mut Physics,
            &mut Sprinting,
            &mut Attributes,
        ),
        With<LocalPlayerInLoadedChunk>,
    >,
) {
    for (mut physics_state, mut physics, mut sprinting, mut attributes) in query.iter_mut() {
        LocalPlayer::tick_controls(None, &mut physics_state);

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
                has_enough_impulse_to_start_sprinting(&physics_state)
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

impl Client {
    /// Start walking in the given direction. To sprint, use
    /// [`Client::sprint`]. To stop walking, call walk with
    /// `WalkDirection::None`.
    ///
    /// # Examples
    ///
    /// Walk for 1 second
    /// ```rust,no_run
    /// # use azalea_client::{Client, WalkDirection};
    /// # use std::time::Duration;
    /// # async fn example(mut bot: Client) {
    /// bot.walk(WalkDirection::Forward);
    /// tokio::time::sleep(Duration::from_secs(1)).await;
    /// bot.walk(WalkDirection::None);
    /// # }
    /// ```
    pub fn walk(&mut self, direction: WalkDirection) {
        let mut ecs = self.ecs.lock();
        ecs.send_event(StartWalkEvent {
            entity: self.entity,
            direction,
        });
    }

    /// Start sprinting in the given direction. To stop moving, call
    /// [`Client::walk(WalkDirection::None)`]
    ///
    /// # Examples
    ///
    /// Sprint for 1 second
    /// ```rust,no_run
    /// # use azalea_client::{Client, WalkDirection, SprintDirection};
    /// # use std::time::Duration;
    /// # async fn example(mut bot: Client) {
    /// bot.sprint(SprintDirection::Forward);
    /// tokio::time::sleep(Duration::from_secs(1)).await;
    /// bot.walk(WalkDirection::None);
    /// # }
    /// ```
    pub fn sprint(&mut self, direction: SprintDirection) {
        let mut ecs = self.ecs.lock();
        ecs.send_event(StartSprintEvent {
            entity: self.entity,
            direction,
        });
    }
}

/// An event sent when the client starts walking. This does not get sent for
/// non-local entities.
#[derive(Event)]
pub struct StartWalkEvent {
    pub entity: Entity,
    pub direction: WalkDirection,
}

/// Start walking in the given direction. To sprint, use
/// [`Client::sprint`]. To stop walking, call walk with
/// `WalkDirection::None`.
pub fn walk_listener(
    mut events: EventReader<StartWalkEvent>,
    mut query: Query<(&mut PhysicsState, &mut Sprinting, &mut Attributes)>,
) {
    for event in events.iter() {
        if let Ok((mut physics_state, mut sprinting, mut attributes)) = query.get_mut(event.entity)
        {
            physics_state.move_direction = event.direction;
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
/// Start sprinting in the given direction.
pub fn sprint_listener(
    mut query: Query<&mut PhysicsState>,
    mut events: EventReader<StartSprintEvent>,
) {
    for event in events.iter() {
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
            .remove(&azalea_entity::attributes::sprinting_modifier().uuid)
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
