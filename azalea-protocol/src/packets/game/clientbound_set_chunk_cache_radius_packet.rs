use azalea_buf::McBuf;
use packet_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundSetChunkCacheRadiusPacket {
    #[var]
    pub radius: u32,
}
