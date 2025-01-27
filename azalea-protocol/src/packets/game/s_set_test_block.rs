use azalea_buf::AzBuf;
use azalea_core::position::BlockPos;
use azalea_protocol_macros::ServerboundGamePacket;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundSetTestBlock {
    pub position: BlockPos,
    pub mode: TestBlockMode,
    pub message: String,
}

#[derive(Clone, Copy, Debug, AzBuf, Default)]
pub enum TestBlockMode {
    #[default]
    Start,
    Log,
    Fail,
    Accept,
}
