use azalea_entity::{metadata::Health, EntityUpdateSet};
use bevy_app::{App, First, Plugin, PreUpdate, Update};
use bevy_ecs::prelude::*;

use crate::chat::ChatReceivedEvent;

use self::{
    game::{
        AddPlayerEvent, DeathEvent, InstanceLoadedEvent, KeepAliveEvent, RemovePlayerEvent,
        ResourcePackEvent, UpdatePlayerEvent,
    },
    login::{LoginPacketEvent, SendLoginPacketEvent},
};

pub mod configuration;
pub mod game;
pub mod login;

pub struct PacketHandlerPlugin;

pub fn death_event_on_0_health(
    query: Query<(Entity, &Health), Changed<Health>>,
    mut death_events: EventWriter<DeathEvent>,
) {
    for (entity, health) in query.iter() {
        if **health == 0. {
            death_events.send(DeathEvent {
                entity,
                packet: None,
            });
        }
    }
}

impl Plugin for PacketHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            First,
            (game::send_packet_events, configuration::send_packet_events),
        )
        .add_systems(
            PreUpdate,
            (
                game::process_packet_events
                    // we want to index and deindex right after
                    .before(EntityUpdateSet::Deindex),
                configuration::process_packet_events,
                login::handle_send_packet_event,
                login::process_packet_events,
            ),
        )
        .add_systems(
            Update,
            (
                (
                    configuration::handle_send_packet_event,
                    game::handle_send_packet_event,
                )
                    .chain(),
                death_event_on_0_health,
            ),
        )
        // we do this instead of add_event so we can handle the events ourselves
        .init_resource::<Events<game::PacketEvent>>()
        .init_resource::<Events<configuration::ConfigurationPacketEvent>>()
        .add_event::<game::SendPacketEvent>()
        .add_event::<configuration::SendConfigurationPacketEvent>()
        .add_event::<AddPlayerEvent>()
        .add_event::<RemovePlayerEvent>()
        .add_event::<UpdatePlayerEvent>()
        .add_event::<ChatReceivedEvent>()
        .add_event::<DeathEvent>()
        .add_event::<KeepAliveEvent>()
        .add_event::<ResourcePackEvent>()
        .add_event::<InstanceLoadedEvent>()
        .add_event::<LoginPacketEvent>()
        .add_event::<SendLoginPacketEvent>();
    }
}
