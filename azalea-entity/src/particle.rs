use azalea_block::BlockState;
use azalea_buf::AzBuf;
use azalea_core::{color::RgbColor, position::BlockPos};
use azalea_inventory::ItemStack;
use azalea_registry::ParticleKind;
use bevy_ecs::component::Component;

// the order of this enum must be kept in sync with ParticleKind, otherwise
// we get errors parsing particles.
/// A [`ParticleKind`] with data potentially attached to it.
#[derive(Component, Clone, Debug, AzBuf, Default)]
pub enum Particle {
    AngryVillager,
    Block(BlockParticle),
    BlockMarker(BlockParticle),
    Bubble,
    Cloud,
    Crit,
    DamageIndicator,
    DragonBreath,
    DrippingLava,
    FallingLava,
    LandingLava,
    DrippingWater,
    FallingWater,
    Dust(DustParticle),
    DustColorTransition(DustColorTransitionParticle),
    Effect,
    ElderGuardian,
    EnchantedHit,
    Enchant,
    EndRod,
    #[default]
    EntityEffect,
    ExplosionEmitter,
    Explosion,
    Gust,
    SmallGust,
    GustEmitterLarge,
    GustEmitterSmall,
    SonicBoom,
    FallingDust(BlockParticle),
    Firework,
    Fishing,
    Flame,
    Infested,
    CherryLeaves,
    PaleOakLeaves,
    TintedLeaves,
    SculkSoul,
    SculkCharge(SculkChargeParticle),
    SculkChargePop,
    SoulFireFlame,
    Soul,
    Flash,
    HappyVillager,
    Composter,
    Heart,
    InstantEffect,
    Item(ItemParticle),
    Vibration(VibrationParticle),
    Trail,
    ItemSlime,
    ItemCobweb,
    ItemSnowball,
    LargeSmoke,
    Lava,
    Mycelium,
    Note,
    Poof,
    Portal,
    Rain,
    Smoke,
    WhiteSmoke,
    Sneeze,
    Spit,
    SquidInk,
    SweepAttack,
    TotemOfUndying,
    Underwater,
    Splash,
    Witch,
    BubblePop,
    CurrentDown,
    BubbleColumnUp,
    Nautilus,
    Dolphin,
    CampfireCosySmoke,
    CampfireSignalSmoke,
    DrippingHoney,
    FallingHoney,
    LandingHoney,
    FallingNectar,
    FallingSporeBlossom,
    Ash,
    CrimsonSpore,
    WarpedSpore,
    SporeBlossomAir,
    DrippingObsidianTear,
    FallingObsidianTear,
    LandingObsidianTear,
    ReversePortal,
    WhiteAsh,
    SmallFlame,
    Snowflake,
    DrippingDripstoneLava,
    FallingDripstoneLava,
    DrippingDripstoneWater,
    FallingDripstoneWater,
    GlowSquidInk,
    Glow,
    WaxOn,
    WaxOff,
    ElectricSpark,
    Scrape,
    Shriek(ShriekParticle),
    EggCrack,
    DustPlume,
    TrialSpawnerDetection,
    TrialSpawnerDetectionOminous,
    VaultConnection,
    DustPillar,
    OminousSpawning,
    RaidOmen,
    TrialOmen,
    BlockCrumble,
}

impl From<ParticleKind> for Particle {
    /// Convert a particle kind into particle data. If the particle has data
    /// attached (like block particles), then it's set to the default.
    fn from(kind: ParticleKind) -> Self {
        // this is mostly just here so it fails to compile when a new particle is added
        // to ParticleKind, since `Particle` has to be updated manually
        match kind {
            ParticleKind::AngryVillager => Self::AngryVillager,
            ParticleKind::Block => Self::Block(BlockParticle::default()),
            ParticleKind::BlockMarker => Self::BlockMarker(BlockParticle::default()),
            ParticleKind::Bubble => Self::Bubble,
            ParticleKind::Cloud => Self::Cloud,
            ParticleKind::Crit => Self::Crit,
            ParticleKind::DamageIndicator => Self::DamageIndicator,
            ParticleKind::DragonBreath => Self::DragonBreath,
            ParticleKind::DrippingLava => Self::DrippingLava,
            ParticleKind::FallingLava => Self::FallingLava,
            ParticleKind::LandingLava => Self::LandingLava,
            ParticleKind::DrippingWater => Self::DrippingWater,
            ParticleKind::FallingWater => Self::FallingWater,
            ParticleKind::Dust => Self::Dust(DustParticle::default()),
            ParticleKind::DustColorTransition => {
                Self::DustColorTransition(DustColorTransitionParticle::default())
            }
            ParticleKind::Effect => Self::Effect,
            ParticleKind::ElderGuardian => Self::ElderGuardian,
            ParticleKind::EnchantedHit => Self::EnchantedHit,
            ParticleKind::Enchant => Self::Enchant,
            ParticleKind::EndRod => Self::EndRod,
            ParticleKind::EntityEffect => Self::EntityEffect,
            ParticleKind::ExplosionEmitter => Self::ExplosionEmitter,
            ParticleKind::Explosion => Self::Explosion,
            ParticleKind::Gust => Self::Gust,
            ParticleKind::SonicBoom => Self::SonicBoom,
            ParticleKind::FallingDust => Self::FallingDust(BlockParticle::default()),
            ParticleKind::Firework => Self::Firework,
            ParticleKind::Fishing => Self::Fishing,
            ParticleKind::Flame => Self::Flame,
            ParticleKind::CherryLeaves => Self::CherryLeaves,
            ParticleKind::PaleOakLeaves => Self::PaleOakLeaves,
            ParticleKind::TintedLeaves => Self::TintedLeaves,
            ParticleKind::SculkSoul => Self::SculkSoul,
            ParticleKind::SculkCharge => Self::SculkCharge(SculkChargeParticle::default()),
            ParticleKind::SculkChargePop => Self::SculkChargePop,
            ParticleKind::SoulFireFlame => Self::SoulFireFlame,
            ParticleKind::Soul => Self::Soul,
            ParticleKind::Flash => Self::Flash,
            ParticleKind::HappyVillager => Self::HappyVillager,
            ParticleKind::Composter => Self::Composter,
            ParticleKind::Heart => Self::Heart,
            ParticleKind::InstantEffect => Self::InstantEffect,
            ParticleKind::Item => Self::Item(ItemParticle::default()),
            ParticleKind::Vibration => Self::Vibration(VibrationParticle::default()),
            ParticleKind::ItemSlime => Self::ItemSlime,
            ParticleKind::ItemSnowball => Self::ItemSnowball,
            ParticleKind::LargeSmoke => Self::LargeSmoke,
            ParticleKind::Lava => Self::Lava,
            ParticleKind::Mycelium => Self::Mycelium,
            ParticleKind::Note => Self::Note,
            ParticleKind::Poof => Self::Poof,
            ParticleKind::Portal => Self::Portal,
            ParticleKind::Rain => Self::Rain,
            ParticleKind::Smoke => Self::Smoke,
            ParticleKind::WhiteSmoke => Self::WhiteSmoke,
            ParticleKind::Sneeze => Self::Sneeze,
            ParticleKind::Spit => Self::Spit,
            ParticleKind::SquidInk => Self::SquidInk,
            ParticleKind::SweepAttack => Self::SweepAttack,
            ParticleKind::TotemOfUndying => Self::TotemOfUndying,
            ParticleKind::Underwater => Self::Underwater,
            ParticleKind::Splash => Self::Splash,
            ParticleKind::Witch => Self::Witch,
            ParticleKind::BubblePop => Self::BubblePop,
            ParticleKind::CurrentDown => Self::CurrentDown,
            ParticleKind::BubbleColumnUp => Self::BubbleColumnUp,
            ParticleKind::Nautilus => Self::Nautilus,
            ParticleKind::Dolphin => Self::Dolphin,
            ParticleKind::CampfireCosySmoke => Self::CampfireCosySmoke,
            ParticleKind::CampfireSignalSmoke => Self::CampfireSignalSmoke,
            ParticleKind::DrippingHoney => Self::DrippingHoney,
            ParticleKind::FallingHoney => Self::FallingHoney,
            ParticleKind::LandingHoney => Self::LandingHoney,
            ParticleKind::FallingNectar => Self::FallingNectar,
            ParticleKind::FallingSporeBlossom => Self::FallingSporeBlossom,
            ParticleKind::Ash => Self::Ash,
            ParticleKind::CrimsonSpore => Self::CrimsonSpore,
            ParticleKind::WarpedSpore => Self::WarpedSpore,
            ParticleKind::SporeBlossomAir => Self::SporeBlossomAir,
            ParticleKind::DrippingObsidianTear => Self::DrippingObsidianTear,
            ParticleKind::FallingObsidianTear => Self::FallingObsidianTear,
            ParticleKind::LandingObsidianTear => Self::LandingObsidianTear,
            ParticleKind::ReversePortal => Self::ReversePortal,
            ParticleKind::WhiteAsh => Self::WhiteAsh,
            ParticleKind::SmallFlame => Self::SmallFlame,
            ParticleKind::Snowflake => Self::Snowflake,
            ParticleKind::DrippingDripstoneLava => Self::DrippingDripstoneLava,
            ParticleKind::FallingDripstoneLava => Self::FallingDripstoneLava,
            ParticleKind::DrippingDripstoneWater => Self::DrippingDripstoneWater,
            ParticleKind::FallingDripstoneWater => Self::FallingDripstoneWater,
            ParticleKind::GlowSquidInk => Self::GlowSquidInk,
            ParticleKind::Glow => Self::Glow,
            ParticleKind::WaxOn => Self::WaxOn,
            ParticleKind::WaxOff => Self::WaxOff,
            ParticleKind::ElectricSpark => Self::ElectricSpark,
            ParticleKind::Scrape => Self::Scrape,
            ParticleKind::Shriek => Self::Shriek(ShriekParticle::default()),
            ParticleKind::EggCrack => Self::EggCrack,
            ParticleKind::DustPlume => Self::DustPlume,
            ParticleKind::SmallGust => Self::SmallGust,
            ParticleKind::GustEmitterLarge => Self::GustEmitterLarge,
            ParticleKind::GustEmitterSmall => Self::GustEmitterSmall,
            ParticleKind::Infested => Self::Infested,
            ParticleKind::ItemCobweb => Self::ItemCobweb,
            ParticleKind::TrialSpawnerDetection => Self::TrialSpawnerDetection,
            ParticleKind::TrialSpawnerDetectionOminous => Self::TrialSpawnerDetectionOminous,
            ParticleKind::VaultConnection => Self::VaultConnection,
            ParticleKind::DustPillar => Self::DustPillar,
            ParticleKind::OminousSpawning => Self::OminousSpawning,
            ParticleKind::RaidOmen => Self::RaidOmen,
            ParticleKind::TrialOmen => Self::TrialOmen,
            ParticleKind::Trail => Self::Trail,
            ParticleKind::BlockCrumble => Self::BlockCrumble,
        }
    }
}

#[derive(Debug, Clone, AzBuf, Default)]
pub struct BlockParticle {
    pub block_state: BlockState,
}
#[derive(Debug, Clone, AzBuf, Default)]
pub struct DustParticle {
    pub color: RgbColor,
    /// The scale, will be clamped between 0.01 and 4.
    pub scale: f32,
}

#[derive(Debug, Clone, AzBuf, Default)]
pub struct DustColorTransitionParticle {
    pub from: RgbColor,
    pub to: RgbColor,
    /// The scale, will be clamped between 0.01 and 4.
    pub scale: f32,
}

#[derive(Debug, Clone, AzBuf, Default)]
pub struct ItemParticle {
    pub item: ItemStack,
}

#[derive(Debug, Clone, AzBuf, Default)]
pub struct VibrationParticle {
    pub origin: BlockPos,
    pub position_type: String,
    pub block_position: BlockPos,
    #[var]
    pub entity_id: u32,
    #[var]
    pub ticks: u32,
}

#[derive(Debug, Clone, AzBuf, Default)]
pub struct SculkChargeParticle {
    pub roll: f32,
}

#[derive(Debug, Clone, AzBuf, Default)]
pub struct ShriekParticle {
    #[var]
    pub delay: i32, // The time in ticks before the particle is displayed
}
