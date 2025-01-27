use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_core::position::{BlockPos, Vec3i};
use azalea_protocol_macros::ServerboundGamePacket;
use azalea_registry::TestInstanceKind;

use super::s_set_structure_block::Rotation;

#[derive(Clone, Debug, AzBuf, ServerboundGamePacket)]
pub struct ServerboundTestInstanceBlockAction {
    pub pos: BlockPos,
    pub action: Action,
    pub data: TestInstanceBlockEntityData,
}

#[derive(Clone, Copy, Debug, AzBuf, Default)]
pub enum Action {
    #[default]
    Init,
    Qurey,
    Set,
    Reset,
    Save,
    Export,
    Run,
}

#[derive(Clone, Debug, AzBuf, Default)]
pub struct TestInstanceBlockEntityData {
    pub test: Option<TestInstanceKind>,
    pub size: Vec3i,
    pub rotation: Rotation,
    pub ignore_entities: bool,
    pub status: TestInstanceBlockEntityStatus,
    pub error_message: Option<FormattedText>,
}

#[derive(Clone, Copy, Debug, AzBuf, Default)]
pub enum TestInstanceBlockEntityStatus {
    #[default]
    Cleared,
    Running,
    Finished,
}
