use std::{io, sync::Arc};

use azalea_auth::game_profile::GameProfile;
use azalea_core::{ChunkPos, GameMode};
use azalea_entity::{Dead, Position};
use azalea_protocol::packets::game::ServerboundGamePacket;
use azalea_world::{Instance, InstanceContainer, InstanceName, PartialInstance};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    event::EventReader,
    prelude::Event,
    query::Added,
    system::{Query, Res},
};
use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use thiserror::Error;
use tokio::{sync::mpsc, task::JoinHandle};

use crate::{
    events::{Event as AzaleaEvent, LocalPlayerEvents},
    ClientInformation,
};

/// This is a component for our local player entities that are probably in a
/// world. If you have access to a [`Client`], you probably don't need to care
/// about this since `Client` gives you access to everything here.
///
/// You can also use the [`Local`] marker component for queries if you're only
/// checking for a local player and don't need the contents of this component.
///
/// [`Local`]: azalea_entity::Local
/// [`Client`]: crate::Client
#[derive(Component)]
pub struct LocalPlayer {
    packet_writer: mpsc::UnboundedSender<ServerboundGamePacket>,

    /// The partial instance is the world this client currently has loaded. It
    /// has a limited render distance.
    pub partial_instance: Arc<RwLock<PartialInstance>>,
    /// The world is the combined [`PartialInstance`]s of all clients in the
    /// same world. (Only relevant if you're using a shared world, i.e. a
    /// swarm)
    pub world: Arc<RwLock<Instance>>,

    /// A task that reads packets from the server. The client is disconnected
    /// when this task ends.
    pub(crate) read_packets_task: JoinHandle<()>,
    /// A task that writes packets from the server.
    pub(crate) write_packets_task: JoinHandle<()>,
}

/// A component only present in players that contains the [`GameProfile`] (which
/// you can use to get a player's name).
///
/// Note that it's possible for this to be missing in a player if the server
/// never sent the player info for them (though this is uncommon).
#[derive(Component, Clone, Debug, Deref, DerefMut)]
pub struct GameProfileComponent(pub GameProfile);

/// Marks a [`LocalPlayer`] that's in a loaded chunk. This is updated at the
/// beginning of every tick.
#[derive(Component, Clone, Debug, Copy)]
pub struct LocalPlayerInLoadedChunk;

/// The gamemode of a local player. For a non-local player, you can look up the
/// player in the [`TabList`].
#[derive(Component, Clone, Debug, Copy)]
pub struct LocalGameMode {
    pub current: GameMode,
    pub previous: Option<GameMode>,
}

impl LocalPlayer {
    /// Create a new `LocalPlayer`.
    pub fn new(
        entity: Entity,
        packet_writer: mpsc::UnboundedSender<ServerboundGamePacket>,
        world: Arc<RwLock<Instance>>,
        read_packets_task: JoinHandle<()>,
        write_packets_task: JoinHandle<()>,
    ) -> Self {
        let client_information = ClientInformation::default();

        LocalPlayer {
            packet_writer,

            world,
            partial_instance: Arc::new(RwLock::new(PartialInstance::new(
                client_information.view_distance.into(),
                Some(entity),
            ))),

            read_packets_task,
            write_packets_task,
        }
    }

    /// Write a packet directly to the server.
    pub fn write_packet(&self, packet: ServerboundGamePacket) {
        self.packet_writer
            .send(packet)
            .expect("write_packet shouldn't be able to be called if the connection is closed");
    }
}

impl Drop for LocalPlayer {
    /// Stop every active task when the `LocalPlayer` is dropped.
    fn drop(&mut self) {
        self.read_packets_task.abort();
        self.write_packets_task.abort();
    }
}

/// Update the [`LocalPlayerInLoadedChunk`] component for all [`LocalPlayer`]s.
pub fn update_in_loaded_chunk(
    mut commands: bevy_ecs::system::Commands,
    query: Query<(Entity, &InstanceName, &Position)>,
    instance_container: Res<InstanceContainer>,
) {
    for (entity, local_player, position) in &query {
        let player_chunk_pos = ChunkPos::from(position);
        let Some(instance_lock) = instance_container.get(local_player) else {
            continue;
        };

        let in_loaded_chunk = instance_lock.read().chunks.get(&player_chunk_pos).is_some();
        if in_loaded_chunk {
            commands.entity(entity).insert(LocalPlayerInLoadedChunk);
        } else {
            commands.entity(entity).remove::<LocalPlayerInLoadedChunk>();
        }
    }
}

/// Send the "Death" event for [`LocalPlayer`]s that died with no reason.
pub fn death_event(query: Query<&LocalPlayerEvents, Added<Dead>>) {
    for local_player_events in &query {
        local_player_events.send(AzaleaEvent::Death(None)).unwrap();
    }
}

#[derive(Error, Debug)]
pub enum HandlePacketError {
    #[error("{0}")]
    Poison(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("{0}")]
    Send(#[from] mpsc::error::SendError<AzaleaEvent>),
}

impl<T> From<std::sync::PoisonError<T>> for HandlePacketError {
    fn from(e: std::sync::PoisonError<T>) -> Self {
        HandlePacketError::Poison(e.to_string())
    }
}

/// Event for sending a packet to the server.
#[derive(Event)]
pub struct SendPacketEvent {
    pub entity: Entity,
    pub packet: ServerboundGamePacket,
}

pub fn handle_send_packet_event(
    mut send_packet_events: EventReader<SendPacketEvent>,
    mut query: Query<&mut LocalPlayer>,
) {
    for event in send_packet_events.iter() {
        if let Ok(local_player) = query.get_mut(event.entity) {
            local_player.write_packet(event.packet.clone());
        }
    }
}
