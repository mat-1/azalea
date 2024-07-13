use std::io::Cursor;

use azalea_entity::indexing::EntityIdIndex;
use azalea_protocol::packets::configuration::serverbound_finish_configuration_packet::ServerboundFinishConfigurationPacket;
use azalea_protocol::packets::configuration::serverbound_keep_alive_packet::ServerboundKeepAlivePacket;
use azalea_protocol::packets::configuration::serverbound_pong_packet::ServerboundPongPacket;
use azalea_protocol::packets::configuration::serverbound_resource_pack_packet::ServerboundResourcePackPacket;
use azalea_protocol::packets::configuration::serverbound_select_known_packs_packet::ServerboundSelectKnownPacksPacket;
use azalea_protocol::packets::configuration::{
    ClientboundConfigurationPacket, ServerboundConfigurationPacket,
};
use azalea_protocol::packets::ConnectionProtocol;
use azalea_protocol::read::deserialize_packet;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use tracing::{debug, error, warn};

use crate::client::InConfigurationState;
use crate::disconnect::DisconnectEvent;
use crate::local_player::Hunger;
use crate::packet_handling::game::KeepAliveEvent;
use crate::raw_connection::RawConnection;
use crate::InstanceHolder;

#[derive(Event, Debug, Clone)]
pub struct ConfigurationPacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: ClientboundConfigurationPacket,
}

pub fn send_packet_events(
    query: Query<(Entity, &RawConnection), With<InConfigurationState>>,
    mut packet_events: ResMut<Events<ConfigurationPacketEvent>>,
) {
    // we manually clear and send the events at the beginning of each update
    // since otherwise it'd cause issues with events in process_packet_events
    // running twice
    packet_events.clear();
    for (player_entity, raw_connection) in &query {
        let packets_lock = raw_connection.incoming_packet_queue();
        let mut packets = packets_lock.lock();
        if !packets.is_empty() {
            for raw_packet in packets.iter() {
                let packet = match deserialize_packet::<ClientboundConfigurationPacket>(
                    &mut Cursor::new(raw_packet),
                ) {
                    Ok(packet) => packet,
                    Err(err) => {
                        error!("failed to read packet: {:?}", err);
                        continue;
                    }
                };
                packet_events.send(ConfigurationPacketEvent {
                    entity: player_entity,
                    packet,
                });
            }
            // clear the packets right after we read them
            packets.clear();
        }
    }
}

pub fn process_packet_events(ecs: &mut World) {
    let mut events_owned = Vec::new();
    let mut system_state: SystemState<EventReader<ConfigurationPacketEvent>> =
        SystemState::new(ecs);
    let mut events = system_state.get_mut(ecs);
    for ConfigurationPacketEvent {
        entity: player_entity,
        packet,
    } in events.read()
    {
        // we do this so `ecs` isn't borrowed for the whole loop
        events_owned.push((*player_entity, packet.clone()));
    }
    for (player_entity, packet) in events_owned {
        match packet {
            ClientboundConfigurationPacket::RegistryData(p) => {
                let mut system_state: SystemState<Query<&mut InstanceHolder>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let instance_holder = query.get_mut(player_entity).unwrap();
                let mut instance = instance_holder.instance.write();

                // add the new registry data
                instance.registries.append(p.registry_id, p.entries);
            }

            ClientboundConfigurationPacket::CustomPayload(p) => {
                debug!("Got custom payload packet {p:?}");
            }
            ClientboundConfigurationPacket::Disconnect(p) => {
                warn!("Got disconnect packet {p:?}");
                let mut system_state: SystemState<EventWriter<DisconnectEvent>> =
                    SystemState::new(ecs);
                let mut disconnect_events = system_state.get_mut(ecs);
                disconnect_events.send(DisconnectEvent {
                    entity: player_entity,
                    reason: Some(p.reason.clone()),
                });
            }
            ClientboundConfigurationPacket::FinishConfiguration(p) => {
                debug!("got FinishConfiguration packet: {p:?}");

                let mut system_state: SystemState<Query<&mut RawConnection>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let mut raw_connection = query.get_mut(player_entity).unwrap();

                raw_connection
                    .write_packet(ServerboundFinishConfigurationPacket {}.get())
                    .expect(
                        "we should be in the right state and encoding this packet shouldn't fail",
                    );
                raw_connection.set_state(ConnectionProtocol::Game);

                // these components are added now that we're going to be in the Game state
                ecs.entity_mut(player_entity)
                    .remove::<InConfigurationState>()
                    .insert(crate::JoinedClientBundle {
                        physics_state: crate::PhysicsState::default(),
                        inventory: crate::inventory::InventoryComponent::default(),
                        tab_list: crate::local_player::TabList::default(),
                        current_sequence_number: crate::interact::CurrentSequenceNumber::default(),
                        last_sent_direction: crate::movement::LastSentLookDirection::default(),
                        abilities: crate::local_player::PlayerAbilities::default(),
                        permission_level: crate::local_player::PermissionLevel::default(),
                        hunger: Hunger::default(),
                        chunk_batch_info: crate::chunks::ChunkBatchInfo::default(),

                        entity_id_index: EntityIdIndex::default(),

                        mining: crate::mining::MineBundle::default(),
                        attack: crate::attack::AttackBundle::default(),

                        _local_entity: azalea_entity::LocalEntity,
                    });
            }
            ClientboundConfigurationPacket::KeepAlive(p) => {
                debug!("Got keep alive packet (in configuration) {p:?} for {player_entity:?}");

                let mut system_state: SystemState<(
                    Query<&RawConnection>,
                    EventWriter<KeepAliveEvent>,
                )> = SystemState::new(ecs);
                let (query, mut keepalive_events) = system_state.get_mut(ecs);
                let raw_connection = query.get(player_entity).unwrap();

                keepalive_events.send(KeepAliveEvent {
                    entity: player_entity,
                    id: p.id,
                });
                raw_connection
                    .write_packet(ServerboundKeepAlivePacket { id: p.id }.get())
                    .unwrap();
            }
            ClientboundConfigurationPacket::Ping(p) => {
                debug!("Got ping packet {p:?}");

                let mut system_state: SystemState<Query<&RawConnection>> = SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let raw_connection = query.get_mut(player_entity).unwrap();

                raw_connection
                    .write_packet(ServerboundPongPacket { id: p.id }.get())
                    .unwrap();
            }
            ClientboundConfigurationPacket::ResourcePackPush(p) => {
                debug!("Got resource pack packet {p:?}");

                let mut system_state: SystemState<Query<&RawConnection>> = SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let raw_connection = query.get_mut(player_entity).unwrap();

                // always accept resource pack
                raw_connection.write_packet(
                    ServerboundResourcePackPacket {
                        id: p.id,
                        action: azalea_protocol::packets::configuration::serverbound_resource_pack_packet::Action::Accepted
                    }.get()
                ).unwrap();
            }
            ClientboundConfigurationPacket::ResourcePackPop(_) => {
                // we can ignore this
            }
            ClientboundConfigurationPacket::UpdateEnabledFeatures(p) => {
                debug!("Got update enabled features packet {p:?}");
            }
            ClientboundConfigurationPacket::UpdateTags(_p) => {
                debug!("Got update tags packet");
            }
            ClientboundConfigurationPacket::CookieRequest(p) => {
                debug!("Got cookie request packet {p:?}");
            }
            ClientboundConfigurationPacket::ResetChat(p) => {
                debug!("Got reset chat packet {p:?}");
            }
            ClientboundConfigurationPacket::StoreCookie(p) => {
                debug!("Got store cookie packet {p:?}");
            }
            ClientboundConfigurationPacket::Transfer(p) => {
                debug!("Got transfer packet {p:?}");
            }
            ClientboundConfigurationPacket::SelectKnownPacks(p) => {
                debug!("Got select known packs packet {p:?}");

                let mut system_state: SystemState<Query<&RawConnection>> = SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let raw_connection = query.get_mut(player_entity).unwrap();

                // resource pack management isn't implemented
                raw_connection
                    .write_packet(
                        ServerboundSelectKnownPacksPacket {
                            known_packs: vec![],
                        }
                        .get(),
                    )
                    .unwrap();
            }
        }
    }
}

/// An event for sending a packet to the server while we're in the
/// `configuration` state.
#[derive(Event)]
pub struct SendConfigurationPacketEvent {
    pub entity: Entity,
    pub packet: ServerboundConfigurationPacket,
}

pub fn handle_send_packet_event(
    mut send_packet_events: EventReader<SendConfigurationPacketEvent>,
    mut query: Query<&mut RawConnection>,
) {
    for event in send_packet_events.read() {
        if let Ok(raw_connection) = query.get_mut(event.entity) {
            debug!("Sending packet: {:?}", event.packet);
            if let Err(e) = raw_connection.write_packet(event.packet.clone()) {
                error!("Failed to send packet: {e}");
            }
        }
    }
}
