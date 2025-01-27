//! Define some types needed for entity metadata.

use std::io::{Cursor, Write};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};
use azalea_chat::FormattedText;
use azalea_core::{
    direction::Direction,
    position::{BlockPos, GlobalPos, Vec3},
};
use azalea_inventory::ItemStack;
use bevy_ecs::component::Component;
use derive_more::Deref;
use enum_as_inner::EnumAsInner;
use uuid::Uuid;

use crate::particle::Particle;

#[derive(Clone, Debug, Deref)]
pub struct EntityMetadataItems(pub Vec<EntityDataItem>);

#[derive(Clone, Debug)]
pub struct EntityDataItem {
    // we can't identify what the index is for here because we don't know the
    // entity type
    pub index: u8,
    pub value: EntityDataValue,
}

impl AzaleaRead for EntityMetadataItems {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let mut metadata = Vec::new();
        loop {
            let id = u8::azalea_read(buf)?;
            if id == 0xff {
                break;
            }
            let value = EntityDataValue::azalea_read(buf)?;
            metadata.push(EntityDataItem { index: id, value });
        }
        Ok(EntityMetadataItems(metadata))
    }
}

impl AzaleaWrite for EntityMetadataItems {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        for item in &self.0 {
            item.index.azalea_write(buf)?;
            item.value.azalea_write(buf)?;
        }
        0xffu8.azalea_write(buf)?;
        Ok(())
    }
}

// Note: This enum is partially generated and parsed by
// codegen/lib/code/entity.py
#[derive(Clone, Debug, EnumAsInner, AzBuf)]
pub enum EntityDataValue {
    Byte(u8),
    Int(#[var] i32),
    Long(#[var] i64),
    Float(f32),
    String(String),
    FormattedText(FormattedText),
    OptionalFormattedText(Option<FormattedText>),
    ItemStack(ItemStack),
    Boolean(bool),
    Rotations(Rotations),
    BlockPos(BlockPos),
    OptionalBlockPos(Option<BlockPos>),
    Direction(Direction),
    OptionalLivingEntityReference(Option<Uuid>),
    BlockState(azalea_block::BlockState),
    /// If this is air, that means it's absent,
    OptionalBlockState(azalea_block::BlockState),
    CompoundTag(simdnbt::owned::NbtCompound),
    Particle(Particle),
    Particles(Vec<Particle>),
    VillagerData(VillagerData),
    // 0 for absent; 1 + actual value otherwise. Used for entity IDs.
    OptionalUnsignedInt(OptionalUnsignedInt),
    Pose(Pose),
    CatVariant(azalea_registry::CatVariant),
    WolfVariant(azalea_registry::WolfVariant),
    FrogVariant(azalea_registry::FrogVariant),
    PigVariant(azalea_registry::PigVariant),
    OptionalGlobalPos(Option<GlobalPos>),
    PaintingVariant(azalea_registry::PaintingVariant),
    SnifferState(SnifferStateKind),
    ArmadilloState(ArmadilloStateKind),
    Vector3(Vec3),
    Quaternion(Quaternion),
}

#[derive(Clone, Debug)]
pub struct OptionalUnsignedInt(pub Option<u32>);

#[derive(Clone, Debug, AzBuf)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

// mojang just calls this ArmadilloState but i added "Kind" since otherwise it
// collides with a name in metadata.rs
#[derive(Clone, Debug, Copy, Default, AzBuf)]
pub enum ArmadilloStateKind {
    #[default]
    Idle,
    Rolling,
    Scared,
}

impl AzaleaRead for OptionalUnsignedInt {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let val = u32::azalea_read_var(buf)?;
        Ok(OptionalUnsignedInt(if val == 0 {
            None
        } else {
            Some(val - 1)
        }))
    }
}
impl AzaleaWrite for OptionalUnsignedInt {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self.0 {
            Some(val) => (val + 1).azalea_write_var(buf),
            None => 0u32.azalea_write_var(buf),
        }
    }
}

/// A set of x, y, and z rotations. This is used for armor stands.
#[derive(Clone, Debug, AzBuf, Default)]
pub struct Rotations {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug, Copy, AzBuf, Default, Component, Eq, PartialEq)]
pub enum Pose {
    #[default]
    Standing = 0,
    FallFlying,
    Sleeping,
    Swimming,
    SpinAttack,
    Sneaking,
    LongJumping,
    Dying,
}

#[derive(Debug, Clone, AzBuf)]
pub struct VillagerData {
    pub kind: azalea_registry::VillagerKind,
    pub profession: azalea_registry::VillagerProfession,
    #[var]
    pub level: u32,
}

#[derive(Debug, Copy, Clone, AzBuf, Default)]
pub enum SnifferStateKind {
    #[default]
    Idling,
    FeelingHappy,
    Scenting,
    Sniffing,
    Searching,
    Digging,
    Rising,
}
