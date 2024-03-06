use crate::client::Client;
use crate::inventory::InventoryComponent;
use crate::local_player::PlayerAbilities;
use crate::packet_handling::game::SendPacketEvent;
use azalea_core::position::Vec3;
use azalea_core::tick::GameTick;
use azalea_entity::metadata::{ShiftKeyDown, Sleeping, SleepingPos, Swimming};
use azalea_entity::{metadata::Sprinting, Attributes, Jumping};
use azalea_entity::{InLoadedChunk, LastSentPosition, LookDirection, Physics, Position, Sneaking};
use azalea_physics::{ai_step, PhysicsSet};
use azalea_protocol::packets::game::serverbound_player_command_packet::ServerboundPlayerCommandPacket;
use azalea_protocol::packets::game::{
    serverbound_move_player_pos_packet::ServerboundMovePlayerPosPacket,
    serverbound_move_player_pos_rot_packet::ServerboundMovePlayerPosRotPacket,
    serverbound_move_player_rot_packet::ServerboundMovePlayerRotPacket,
    serverbound_move_player_status_only_packet::ServerboundMovePlayerStatusOnlyPacket,
};
use azalea_world::{InstanceContainer, InstanceName, MinecraftEntityId, MoveEntityError};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::{Event, EventWriter};
use bevy_ecs::schedule::SystemSet;
use bevy_ecs::system::Res;
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
                    (update_sneaking, tick_controls, local_player_ai_step)
                        .chain()
                        .in_set(PhysicsSet)
                        .before(ai_step),
                    (send_sprinting_if_needed, send_shift_key_down_if_needed)
                        .chain()
                        .after(azalea_entity::update_in_loaded_chunk),
                    send_position.after(PhysicsSet),
                )
                    .chain(),
            );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MoveEventsSet;

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
        *self.component::<Jumping>()
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

    /// Returns the direction the client is looking. The first value is the y
    /// rotation (ie. yaw, looking to the side) and the second value is the x
    /// rotation (ie. pitch, looking up and down).
    pub fn direction(&self) -> (f32, f32) {
        let look_direction = self.component::<LookDirection>();
        (look_direction.y_rot, look_direction.x_rot)
    }
}

/// A component that contains the look direction that was last sent over the
/// network.
#[derive(Debug, Component, Clone, Default)]
pub struct LastSentLookDirection {
    pub x_rot: f32,
    pub y_rot: f32,
}

/// Component for entities that can move, sprint, and sneak. Usually only in
/// [`LocalEntity`]s.
///
/// [`LocalEntity`]: azalea_entity::LocalEntity
#[derive(Default, Component, Clone)]
pub struct PhysicsState {
    /// Minecraft only sends a movement packet either after 20 ticks or if the
    /// player moved enough. This is that tick counter.
    pub position_remainder: u32,

    pub was_sprinting: bool,

    /// Whether the player was sneaking last tick. You shouldn't modify this.
    ///
    /// If you want to check or change the player's sneak state from the ECS,
    /// use the [`ShiftKeyDown`] component.
    pub was_sneaking: bool,

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
    for (entity, &minecraft_entity_id, &Sprinting(sprinting), mut physics_state) in query.iter_mut()
    {
        let was_sprinting = physics_state.was_sprinting;
        if sprinting != was_sprinting {
            let sprinting_action = if sprinting {
                azalea_protocol::packets::game::serverbound_player_command_packet::Action::StartSprinting
            } else {
                azalea_protocol::packets::game::serverbound_player_command_packet::Action::StopSprinting
            };
            send_packet_events.send(SendPacketEvent {
                entity,
                packet: ServerboundPlayerCommandPacket {
                    id: *minecraft_entity_id,
                    action: sprinting_action,
                    data: 0,
                }
                .get(),
            });
            physics_state.was_sprinting = sprinting;
        }
    }
}

fn send_shift_key_down_if_needed(
    mut query: Query<(Entity, &MinecraftEntityId, &ShiftKeyDown, &mut PhysicsState)>,
    mut send_packet_events: EventWriter<SendPacketEvent>,
) {
    for (entity, &minecraft_entity_id, &ShiftKeyDown(sneaking), mut physics_state) in
        query.iter_mut()
    {
        let was_sneaking = physics_state.was_sneaking;
        if sneaking != was_sneaking {
            let sneaking_action = if sneaking {
                azalea_protocol::packets::game::serverbound_player_command_packet::Action::PressShiftKey
            } else {
                azalea_protocol::packets::game::serverbound_player_command_packet::Action::ReleaseShiftKey
            };
            send_packet_events.send(SendPacketEvent {
                entity,
                packet: ServerboundPlayerCommandPacket {
                    id: *minecraft_entity_id,
                    action: sneaking_action,
                    data: 0,
                }
                .get(),
            });
            physics_state.was_sneaking = sneaking;
        }
    }
}

fn can_player_fit_within_blocks_and_entities_when(
    _pose: azalea_entity::Pose,
    _position: Vec3,
    _instance: &azalea_world::Instance,
) -> bool {
    // TODO
    true
}

pub fn update_sneaking(
    mut query: Query<(
        &PlayerAbilities,
        &Swimming,
        // TODO: isPassenger
        &ShiftKeyDown,
        &SleepingPos,
        &Position,
        &InstanceName,
        &mut Sneaking,
    )>,
    instances: Res<InstanceContainer>,
) {
    for (
        player_abilities,
        &Swimming(swimming),
        &ShiftKeyDown(shift_key_down),
        &SleepingPos(sleeping_pos),
        &Position(position),
        instance_name,
        mut sneaking,
    ) in query.iter_mut()
    {
        let Some(instance) = instances.get(instance_name) else {
            continue;
        };
        // this.crouching = !this.getAbilities().flying
        // && !this.isSwimming()
        // && !this.isPassenger()
        // && this.canPlayerFitWithinBlocksAndEntitiesWhen(Pose.CROUCHING)
        // && (this.isShiftKeyDown()
        //       || !this.isSleeping() && !this.canPlayerFitWithinBlocksAndEntitiesWhen(Pose.STANDING));

        let instance = instance.read();

        **sneaking = !player_abilities.flying
            && !swimming
            // && !isPassenger
            // && this.canPlayerFitWithinBlocksAndEntitiesWhen(Pose.CROUCHING)
            && can_player_fit_within_blocks_and_entities_when(
                azalea_entity::Pose::Crouching,
                position,
                &instance,
            )
             && (shift_key_down
                || (
                    sleeping_pos.is_none()
                    && !can_player_fit_within_blocks_and_entities_when(azalea_entity::Pose::Standing, position, &instance)
                )
            );
    }
}

fn is_moving_slowly(sneaking: bool, is_visually_crawling: bool) -> bool {
    sneaking || is_visually_crawling
}

/// Update the impulse from self.move_direction. The multiplier is used for
/// sneaking.
pub(crate) fn tick_controls(mut query: Query<(&mut PhysicsState, &Sneaking, &InventoryComponent)>) {
    for (mut physics_state, &Sneaking(sneaking), inventory) in query.iter_mut() {
        let multiplier: Option<f32> = if is_moving_slowly(sneaking, false) {
            let sneaking_speed_bonus =
                azalea_entity::enchantments::get_sneaking_speed_bonus(&inventory.inventory_menu);
            Some(f32::clamp(0.3 + sneaking_speed_bonus, 0., 1.))
        } else {
            None
        };

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

    /// Start or stop sneaking.
    ///
    /// ```rust,no_run
    /// # use azalea_client::{Client, WalkDirection, SprintDirection};
    /// # use std::time::Duration;
    /// # async fn example(mut bot: Client) {
    /// // toggle sneak
    /// bot.sneak(!bot.sneaking());
    /// # }
    /// ```
    pub fn sneak(&mut self, sneaking: bool) {
        let mut ecs = self.ecs.lock();
        let mut sneaking_component = self.query::<&mut ShiftKeyDown>(&mut ecs);
        **sneaking_component = sneaking;
    }

    /// Returns whether the player is sneaking.
    pub fn sneaking(&self) -> bool {
        *self.component::<ShiftKeyDown>()
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
