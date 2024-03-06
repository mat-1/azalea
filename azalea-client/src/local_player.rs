use std::{collections::HashMap, io, sync::Arc};

use azalea_auth::game_profile::GameProfile;
use azalea_core::game_type::GameMode;
use azalea_entity::Dead;
use azalea_protocol::packets::game::clientbound_player_abilities_packet::ClientboundPlayerAbilitiesPacket;
use azalea_world::{Instance, PartialInstance};
use bevy_ecs::{component::Component, prelude::*};
use derive_more::{Deref, DerefMut};
use parking_lot::RwLock;
use thiserror::Error;
use tokio::sync::mpsc;
use tracing::error;
use uuid::Uuid;

use crate::{
    events::{Event as AzaleaEvent, LocalPlayerEvents},
    ClientInformation, PlayerInfo,
};

/// A component that keeps strong references to our [`PartialInstance`] and
/// [`Instance`] for local players.
#[derive(Component, Clone)]
pub struct InstanceHolder {
    /// The partial instance is the world this client currently has loaded. It
    /// has a limited render distance.
    pub partial_instance: Arc<RwLock<PartialInstance>>,
    /// The world is the combined [`PartialInstance`]s of all clients in the
    /// same world. (Only relevant if you're using a shared world, i.e. a
    /// swarm)
    pub instance: Arc<RwLock<Instance>>,
}

/// A component only present in players that contains the [`GameProfile`] (which
/// you can use to get a player's name).
///
/// Note that it's possible for this to be missing in a player if the server
/// never sent the player info for them (though this is uncommon).
#[derive(Component, Clone, Debug, Deref, DerefMut)]
pub struct GameProfileComponent(pub GameProfile);

/// The gamemode of a local player. For a non-local player, you can look up the
/// player in the [`TabList`].
#[derive(Component, Clone, Debug, Copy)]
pub struct LocalGameMode {
    pub current: GameMode,
    pub previous: Option<GameMode>,
}

impl From<&ClientboundPlayerAbilitiesPacket> for PlayerAbilities {
    fn from(packet: &ClientboundPlayerAbilitiesPacket) -> Self {
        Self {
            invulnerable: packet.flags.invulnerable,
            flying: packet.flags.flying,
            can_fly: packet.flags.can_fly,
            instant_break: packet.flags.instant_break,
            flying_speed: packet.flying_speed,
            walking_speed: packet.walking_speed,
        }
    }
}

/// Level must be 0..=4
#[derive(Component, Clone, Default, Deref, DerefMut)]
pub struct PermissionLevel(pub u8);

/// A component that contains a map of player UUIDs to their information in the
/// tab list.
///
/// ```
/// # use azalea_client::TabList;
/// # fn example(client: &azalea_client::Client) {
/// let tab_list = client.component::<TabList>();
/// println!("Online players:");
/// for (uuid, player_info) in tab_list.iter() {
///     println!("- {} ({}ms)", player_info.profile.name, player_info.latency);
/// }
/// # }
#[derive(Component, Resource, Clone, Debug, Deref, DerefMut, Default)]
pub struct TabList(HashMap<Uuid, PlayerInfo>);

#[derive(Component, Clone)]
pub struct Hunger {
    /// The main hunger bar. Goes from 0 to 20.
    pub food: u32,
    /// The amount of saturation the player has. This isn't shown in normal
    /// vanilla clients but it's a separate counter that makes it so your hunger
    /// only starts decreasing when this is 0.
    pub saturation: f32,
}

impl Default for Hunger {
    fn default() -> Self {
        Hunger {
            food: 20,
            saturation: 5.,
        }
    }
}

impl InstanceHolder {
    /// Create a new `InstanceHolder`.
    pub fn new(entity: Entity, world: Arc<RwLock<Instance>>) -> Self {
        let client_information = ClientInformation::default();

        InstanceHolder {
            instance: world,
            partial_instance: Arc::new(RwLock::new(PartialInstance::new(
                azalea_world::chunk_storage::calculate_chunk_storage_range(
                    client_information.view_distance.into(),
                ),
                Some(entity),
            ))),
        }
    }
}

/// Send the "Death" event for [`LocalEntity`]s that died with no reason.
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
