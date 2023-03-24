use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundTagQueryPacket {
    #[var]
    pub transaction_id: u32,
    pub tag: azalea_nbt::NbtTag,
}
