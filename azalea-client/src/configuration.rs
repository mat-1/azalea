use azalea_buf::McBufWritable;
use azalea_core::{registry_holder::RegistryHolder, resource_location::ResourceLocation};
use azalea_protocol::packets::configuration::{
    serverbound_client_information_packet::{
        ClientInformation, ServerboundClientInformationPacket,
    },
    serverbound_custom_payload_packet::ServerboundCustomPayloadPacket,
};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use crate::{
    client::InConfigurationState, packet_handling::configuration::SendConfigurationPacketEvent,
};

pub struct ConfigurationPlugin;
impl Plugin for ConfigurationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_in_configuration_state
                .after(crate::packet_handling::configuration::handle_send_packet_event),
        );
    }
}

/// The registry data that's been collected while in the configuration state. If
/// you just want to get the client's registries, use [`Instance::registries`]
/// instead.
///
/// This component isn't inserted until we get a
/// [`ClientboundRegistryDataPacket`].
///
/// [`Instance::registries`]: azalea_world::Instance::registries
/// [`ClientboundRegistryDataPacket`]: azalea_protocol::packets::configuration::clientbound_registry_data_packet::ClientboundRegistryDataPacket
#[derive(Component, Debug, Default)]
pub struct RegistryDataCollected {
    pub holder: RegistryHolder,
}

fn handle_in_configuration_state(
    query: Query<(Entity, &ClientInformation), Added<InConfigurationState>>,
    mut send_packet_events: EventWriter<SendConfigurationPacketEvent>,
) {
    for (entity, client_information) in query.iter() {
        let mut brand_data = Vec::new();
        // they don't have to know :)
        "vanilla".write_into(&mut brand_data).unwrap();
        send_packet_events.send(SendConfigurationPacketEvent {
            entity,
            packet: ServerboundCustomPayloadPacket {
                identifier: ResourceLocation::new("brand"),
                data: brand_data.into(),
            }
            .get(),
        });

        send_packet_events.send(SendConfigurationPacketEvent {
            entity,
            packet: ServerboundClientInformationPacket {
                information: client_information.clone(),
            }
            .get(),
        });
    }
}
