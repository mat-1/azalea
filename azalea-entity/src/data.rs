//! Define some types needed for entity metadata.

use azalea_buf::{
    BufReadError, McBuf, McBufReadable, McBufVarReadable, McBufVarWritable, McBufWritable,
};
use azalea_chat::FormattedText;
use azalea_core::{
    direction::Direction,
    particle::Particle,
    position::{BlockPos, GlobalPos, Vec3},
};
use azalea_inventory::ItemSlot;
use bevy_ecs::component::Component;
use derive_more::Deref;
use enum_as_inner::EnumAsInner;
use nohash_hasher::IntSet;
use std::io::{Cursor, Write};
use uuid::Uuid;

#[derive(Clone, Debug, Deref)]
pub struct EntityMetadataItems(Vec<EntityDataItem>);

#[derive(Clone, Debug)]
pub struct EntityDataItem {
    // we can't identify what the index is for here because we don't know the
    // entity type
    pub index: u8,
    pub value: EntityDataValue,
}

impl EntityMetadataItems {
    pub fn new(data: Vec<EntityDataItem>) -> Self {
        EntityMetadataItems(data)
    }
}

impl McBufReadable for EntityMetadataItems {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let mut metadata = Vec::new();
        loop {
            let id = u8::read_from(buf)?;
            if id == 0xff {
                break;
            }
            let value = EntityDataValue::read_from(buf)?;
            metadata.push(EntityDataItem { index: id, value });
        }
        Ok(EntityMetadataItems(metadata))
    }
}

impl McBufWritable for EntityMetadataItems {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        for item in &self.0 {
            item.index.write_into(buf)?;
            item.value.write_into(buf)?;
        }
        0xffu8.write_into(buf)?;
        Ok(())
    }
}

// Note: This enum is partially generated and parsed by
// codegen/lib/code/entity.py
#[derive(Clone, Debug, EnumAsInner, McBuf)]
pub enum EntityDataValue {
    Byte(u8),
    Int(#[var] i32),
    Long(i64),
    Float(f32),
    String(String),
    FormattedText(FormattedText),
    OptionalFormattedText(Option<FormattedText>),
    ItemStack(ItemSlot),
    Boolean(bool),
    Rotations(Rotations),
    BlockPos(BlockPos),
    OptionalBlockPos(Option<BlockPos>),
    Direction(Direction),
    OptionalUuid(Option<Uuid>),
    BlockState(azalea_block::BlockState),
    /// If this is air, that means it's absent,
    OptionalBlockState(azalea_block::BlockState),
    CompoundTag(simdnbt::owned::NbtCompound),
    Particle(Particle),
    VillagerData(VillagerData),
    // 0 for absent; 1 + actual value otherwise. Used for entity IDs.
    OptionalUnsignedInt(OptionalUnsignedInt),
    Pose(Pose),
    CatVariant(azalea_registry::CatVariant),
    FrogVariant(azalea_registry::FrogVariant),
    OptionalGlobalPos(Option<GlobalPos>),
    PaintingVariant(azalea_registry::PaintingVariant),
    SnifferState(SnifferState),
    Vector3(Vec3),
    Quaternion(Quaternion),
}

#[derive(Clone, Debug)]
pub struct OptionalUnsignedInt(pub Option<u32>);

#[derive(Clone, Debug, McBuf)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl McBufReadable for OptionalUnsignedInt {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let val = u32::var_read_from(buf)?;
        Ok(OptionalUnsignedInt(if val == 0 {
            None
        } else {
            Some(val - 1)
        }))
    }
}
impl McBufWritable for OptionalUnsignedInt {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self.0 {
            Some(val) => (val + 1).var_write_into(buf),
            None => 0u32.var_write_into(buf),
        }
    }
}

/// A set of x, y, and z rotations. This is used for armor stands.
#[derive(Clone, Debug, McBuf, Default)]
pub struct Rotations {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug, Copy, McBuf, Default, Component, Eq, PartialEq)]
pub enum Pose {
    #[default]
    Standing = 0,
    FallFlying,
    Sleeping,
    Swimming,
    SpinAttack,
    Crouching,
    LongJumping,
    Dying,
    Croaking,
    UsingTongue,
    Sitting,
    Roaring,
    Sniffing,
    Emerging,
    Digging,
    Sliding,
    Shooting,
    Inhaling,
}

#[derive(Debug, Clone, McBuf)]
pub struct VillagerData {
    pub kind: azalea_registry::VillagerKind,
    pub profession: azalea_registry::VillagerProfession,
    #[var]
    pub level: u32,
}

impl TryFrom<EntityMetadataItems> for Vec<EntityDataValue> {
    type Error = String;

    fn try_from(data: EntityMetadataItems) -> Result<Self, Self::Error> {
        let mut data = data.0;

        data.sort_by(|a, b| a.index.cmp(&b.index));

        let mut prev_indexes = IntSet::default();
        let len = data.len();
        // check to make sure it's valid, in vanilla this is guaranteed to pass
        // but it's possible there's mods that mess with it so we want to make
        // sure it's good
        for item in &data {
            if prev_indexes.contains(&item.index) {
                return Err(format!("Index {} is duplicated", item.index));
            }
            if item.index as usize > len {
                return Err(format!("Index {} is too big", item.index));
            }
            prev_indexes.insert(item.index);
        }

        let data = data.into_iter().map(|d| d.value).collect();

        Ok(data)
    }
}

#[derive(Debug, Copy, Clone, McBuf, Default)]
pub enum SnifferState {
    #[default]
    Idling,
    FeelingHappy,
    Scenting,
    Sniffing,
    Searching,
    Digging,
    Rising,
}
