use azalea_buf::McBuf;
use azalea_nbt::Nbt;
use azalea_protocol_macros::ClientboundGamePacket;

use super::clientbound_light_update_packet::ClientboundLightUpdatePacketData;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundLevelChunkWithLightPacket {
    pub x: i32,
    pub z: i32,
    pub chunk_data: ClientboundLevelChunkPacketData,
    pub light_data: ClientboundLightUpdatePacketData,
}

#[derive(Clone, Debug, McBuf)]
pub struct ClientboundLevelChunkPacketData {
    pub heightmaps: Nbt,
    // we can't parse the data in azalea-protocol because it depends on context from other packets
    pub data: Vec<u8>,
    pub block_entities: Vec<BlockEntity>,
}

#[derive(Clone, Debug, McBuf)]
pub struct BlockEntity {
    pub packed_xz: u8,
    pub y: u16,
    pub kind: azalea_registry::BlockEntityKind,
    pub data: Nbt,
}
