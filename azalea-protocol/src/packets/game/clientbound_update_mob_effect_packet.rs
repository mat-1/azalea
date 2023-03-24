use azalea_buf::McBuf;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, McBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateMobEffectPacket {
    #[var]
    pub entity_id: u32,
    pub effect: azalea_registry::MobEffect,
    pub effect_amplifier: u8,
    #[var]
    pub effect_duration_ticks: u32,
    pub flags: u8,
    pub factor_data: Option<azalea_nbt::NbtTag>,
}
