use azalea_core::GameMode;
use azalea_entity::{
    metadata::{ShiftKeyDown, Sprinting},
    Attributes, Physics,
};
use azalea_physics::PhysicsSet;
use azalea_protocol::packets::game::serverbound_interact_packet::{
    self, ServerboundInteractPacket,
};
use azalea_world::MinecraftEntityId;
use bevy_app::{App, FixedUpdate, Plugin, Update};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};

use crate::{
    interact::SwingArmEvent,
    local_player::{LocalGameMode, SendPacketEvent},
    Client,
};

pub struct AttackPlugin;
impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>()
            .add_systems(
                Update,
                handle_attack_event
                    .after(crate::movement::walk_listener)
                    .before(crate::interact::handle_swing_arm_event)
                    .before(azalea_physics::handle_force_jump)
                    .after(crate::respawn::perform_respawn),
            )
            .add_systems(
                FixedUpdate,
                (
                    increment_ticks_since_last_attack,
                    update_attack_strength_scale.after(PhysicsSet),
                )
                    .chain(),
            );
    }
}

impl Client {
    /// Attack the entity with the given id.
    pub fn attack(&mut self, entity_id: MinecraftEntityId) {
        self.ecs.lock().send_event(AttackEvent {
            entity: self.entity,
            target: entity_id,
        });
    }

    /// Whether the player has an attack cooldown.
    pub fn has_attack_cooldown(&self) -> bool {
        let ticks_since_last_attack = *self.component::<AttackStrengthScale>();
        ticks_since_last_attack < 1.0
    }
}

#[derive(Event)]
pub struct AttackEvent {
    pub entity: Entity,
    pub target: MinecraftEntityId,
}
pub fn handle_attack_event(
    mut events: EventReader<AttackEvent>,
    mut query: Query<(
        &LocalGameMode,
        &mut TicksSinceLastAttack,
        &mut Physics,
        &mut Sprinting,
        &mut ShiftKeyDown,
    )>,
    mut send_packet_events: EventWriter<SendPacketEvent>,
    mut swing_arm_event: EventWriter<SwingArmEvent>,
) {
    for event in events.iter() {
        let (game_mode, mut ticks_since_last_attack, mut physics, mut sprinting, sneaking) =
            query.get_mut(event.entity).unwrap();

        swing_arm_event.send(SwingArmEvent {
            entity: event.entity,
        });
        send_packet_events.send(SendPacketEvent {
            entity: event.entity,
            packet: ServerboundInteractPacket {
                entity_id: *event.target,
                action: serverbound_interact_packet::ActionType::Attack,
                using_secondary_action: **sneaking,
            }
            .get(),
        });

        // we can't attack if we're in spectator mode but it still sends the attack
        // packet
        if game_mode.current == GameMode::Spectator {
            continue;
        };

        ticks_since_last_attack.0 = 0;

        physics.delta = physics.delta.multiply(0.6, 1.0, 0.6);
        **sprinting = false;
    }
}

#[derive(Default, Bundle)]
pub struct AttackBundle {
    pub ticks_since_last_attack: TicksSinceLastAttack,
    pub attack_strength_scale: AttackStrengthScale,
}

#[derive(Default, Component, Clone, Deref, DerefMut)]
pub struct TicksSinceLastAttack(pub u32);
pub fn increment_ticks_since_last_attack(mut query: Query<&mut TicksSinceLastAttack>) {
    for mut ticks_since_last_attack in query.iter_mut() {
        **ticks_since_last_attack += 1;
    }
}

#[derive(Default, Component, Clone, Deref, DerefMut)]
pub struct AttackStrengthScale(pub f32);
pub fn update_attack_strength_scale(
    mut query: Query<(&TicksSinceLastAttack, &Attributes, &mut AttackStrengthScale)>,
) {
    for (ticks_since_last_attack, attributes, mut attack_strength_scale) in query.iter_mut() {
        // look 0.5 ticks into the future because that's what vanilla does
        **attack_strength_scale =
            get_attack_strength_scale(ticks_since_last_attack.0, attributes, 0.5);
    }
}

/// Returns how long it takes for the attack cooldown to reset (in ticks).
pub fn get_attack_strength_delay(attributes: &Attributes) -> f32 {
    ((1. / attributes.attack_speed.calculate()) * 20.) as f32
}

pub fn get_attack_strength_scale(
    ticks_since_last_attack: u32,
    attributes: &Attributes,
    in_ticks: f32,
) -> f32 {
    let attack_strength_delay = get_attack_strength_delay(attributes);
    let attack_strength = (ticks_since_last_attack as f32 + in_ticks) / attack_strength_delay;
    attack_strength.clamp(0., 1.)
}
