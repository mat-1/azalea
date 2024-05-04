use azalea_buf::McBuf;
use azalea_core::{registry_holder::PackedRegistryEntry, resource_location::ResourceLocation};
use azalea_protocol_macros::ClientboundConfigurationPacket;

#[derive(Clone, Debug, McBuf, ClientboundConfigurationPacket)]
pub struct ClientboundRegistryDataPacket {
    pub registry_id: ResourceLocation,
    // this is a vec because the order is significant
    pub entries: Vec<PackedRegistryEntry>,
}
