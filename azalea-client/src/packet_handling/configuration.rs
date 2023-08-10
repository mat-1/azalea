use std::io::Cursor;
use std::sync::Arc;

use azalea_protocol::packets::configuration::serverbound_finish_configuration_packet::ServerboundFinishConfigurationPacket;
use azalea_protocol::packets::configuration::ClientboundConfigurationPacket;
use azalea_protocol::packets::ConnectionProtocol;
use azalea_protocol::read::deserialize_packet;
use azalea_world::Instance;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use log::error;
use parking_lot::RwLock;

use crate::client::InConfigurationState;
use crate::local_player::SendPacketEvent;
use crate::raw_connection::RawConnection;
use crate::ReceivedRegistries;

#[derive(Event, Debug, Clone)]
pub struct PacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: ClientboundConfigurationPacket,
}

pub fn send_packet_events(
    query: Query<(Entity, &RawConnection), With<InConfigurationState>>,
    mut packet_events: ResMut<Events<PacketEvent>>,
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
                packet_events.send(PacketEvent {
                    entity: player_entity,
                    packet: packet.clone(),
                });
            }
            // clear the packets right after we read them
            packets.clear();
        }
    }
}

pub fn process_packet_events(ecs: &mut World) {
    let mut events_owned = Vec::new();
    let mut system_state: SystemState<EventReader<PacketEvent>> = SystemState::new(ecs);
    let mut events = system_state.get_mut(ecs);
    for PacketEvent {
        entity: player_entity,
        packet,
    } in events.iter()
    {
        // we do this so `ecs` isn't borrowed for the whole loop
        events_owned.push((*player_entity, packet.clone()));
    }
    for (player_entity, packet) in events_owned {
        match packet {
            ClientboundConfigurationPacket::RegistryData(p) => {
                let mut system_state: SystemState<Query<&mut ReceivedRegistries>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let mut received_registries = query.get_mut(player_entity).unwrap();

                let new_received_registries = p.registry_holder.registries;
                // override the old registries with the new ones
                // but if a registry wasn't sent, keep the old one
                for (registry_name, registry) in new_received_registries {
                    received_registries
                        .registries
                        .insert(registry_name, registry);
                }
            }

            ClientboundConfigurationPacket::CustomPayload(_) => {}
            ClientboundConfigurationPacket::Disconnect(_) => {}
            ClientboundConfigurationPacket::FinishConfiguration(p) => {
                println!("got FinishConfiguration packet: {p:?}");

                let mut system_state: SystemState<Query<&mut RawConnection>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let mut raw_connection = query.get_mut(player_entity).unwrap();

                let instance_holder = crate::local_player::InstanceHolder::new(
                    player_entity,
                    // default to an empty world, it'll be set correctly later when we
                    // get the login packet
                    Arc::new(RwLock::new(Instance::default())),
                );

                raw_connection.write_packet(&ServerboundFinishConfigurationPacket {}.get());

                raw_connection.set_state(ConnectionProtocol::Game);

                // these components are added now that we're going to be in the Game state
                ecs.entity_mut(player_entity)
                    .remove::<InConfigurationState>()
                    .insert(crate::JoinedClientBundle {
                        instance_holder,
                        physics_state: crate::local_player::PhysicsState::default(),
                        inventory: crate::inventory::InventoryComponent::default(),
                        client_information: crate::ClientInformation::default(),
                        tab_list: crate::TabList::default(),
                        current_sequence_number: crate::interact::CurrentSequenceNumber::default(),
                        last_sent_direction: crate::movement::LastSentLookDirection::default(),
                        abilities: crate::local_player::PlayerAbilities::default(),
                        permission_level: crate::local_player::PermissionLevel::default(),
                        mining: crate::mining::MineBundle::default(),
                        attack: crate::attack::AttackBundle::default(),
                        chunk_batch_info: crate::chunk_batching::ChunkBatchInfo::default(),
                        _local_entity: azalea_entity::LocalEntity,
                    });
            }
            ClientboundConfigurationPacket::KeepAlive(_) => {}
            ClientboundConfigurationPacket::Ping(_) => {}
            ClientboundConfigurationPacket::ResourcePack(_) => {}
            ClientboundConfigurationPacket::UpdateEnabledFeatures(_) => {}
            ClientboundConfigurationPacket::UpdateTags(_) => {}
        }
    }
}
