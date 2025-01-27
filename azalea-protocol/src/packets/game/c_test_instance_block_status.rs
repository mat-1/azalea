use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_core::position::Vec3;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundTestInstanceBlockStatus {
    pub status: FormattedText,
    pub size: Option<Vec3>,
}
