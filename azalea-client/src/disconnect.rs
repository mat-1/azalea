//! Disconnect a client from the server.

use azalea_chat::FormattedText;
use azalea_entity::LocalEntity;
use bevy_app::{App, Plugin, PostUpdate};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::{EventReader, EventWriter},
    prelude::Event,
    query::{Changed, With},
    schedule::IntoSystemConfigs,
    system::{Commands, Query},
};
use derive_more::Deref;

use crate::{client::JoinedClientBundle, raw_connection::RawConnection};

pub struct DisconnectPlugin;
impl Plugin for DisconnectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DisconnectEvent>().add_systems(
            PostUpdate,
            (
                update_read_packets_task_running_component,
                disconnect_on_connection_dead,
                remove_components_from_disconnected_players,
            )
                .chain(),
        );
    }
}

/// An event sent when a client is getting disconnected.
#[derive(Event)]
pub struct DisconnectEvent {
    pub entity: Entity,
    pub reason: Option<FormattedText>,
}

/// System that removes the [`JoinedClientBundle`] from the entity when it
/// receives a [`DisconnectEvent`].
pub fn remove_components_from_disconnected_players(
    mut commands: Commands,
    mut events: EventReader<DisconnectEvent>,
) {
    for DisconnectEvent { entity, .. } in events.read() {
        commands.entity(*entity).remove::<JoinedClientBundle>();
    }
}

#[derive(Component, Clone, Copy, Debug, Deref)]
pub struct IsConnectionAlive(bool);

fn update_read_packets_task_running_component(
    query: Query<(Entity, &RawConnection)>,
    mut commands: Commands,
) {
    for (entity, raw_connection) in &query {
        let running = raw_connection.is_alive();
        commands.entity(entity).insert(IsConnectionAlive(running));
    }
}

#[allow(clippy::type_complexity)]
fn disconnect_on_connection_dead(
    query: Query<(Entity, &IsConnectionAlive), (Changed<IsConnectionAlive>, With<LocalEntity>)>,
    mut disconnect_events: EventWriter<DisconnectEvent>,
) {
    for (entity, &is_connection_alive) in &query {
        if !*is_connection_alive {
            disconnect_events.send(DisconnectEvent {
                entity,
                reason: None,
            });
        }
    }
}
