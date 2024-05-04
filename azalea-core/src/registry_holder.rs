//! The data sent to the client in the `ClientboundRegistryDataPacket`.
//!
//! This module contains the structures used to represent the registry
//! sent to the client upon login. This contains a lot of information about
//! the game, including the types of chat messages, dimensions, and
//! biomes.

use azalea_buf::McBuf;
use simdnbt::{
    owned::{NbtCompound, NbtTag},
    Deserialize, FromNbtTag, Serialize, ToNbtTag,
};
use std::{collections::HashMap, io::Cursor};
use tracing::error;

use crate::resource_location::ResourceLocation;

/// The base of the registry.
///
/// This is the registry that is sent to the client upon login.
#[derive(Default, Debug, Clone)]
pub struct RegistryHolder {
    pub map: HashMap<ResourceLocation, Vec<PackedRegistryEntry>>,
}

#[derive(Clone, Debug, McBuf)]
pub struct PackedRegistryEntry {
    pub id: ResourceLocation,
    pub data: Option<NbtCompound>,
}

impl RegistryHolder {
    pub fn append(&mut self, id: ResourceLocation, entries: Vec<PackedRegistryEntry>) {
        // yes, duplicates are allowed, for some reason.
        self.map.entry(id).or_default().extend(entries);
    }

    fn get<T: Deserialize>(
        &self,
        name: &ResourceLocation,
    ) -> Option<Result<RegistryType<T>, simdnbt::DeserializeError>> {
        // this is suboptimal, ideally simdnbt should just have a way to get the
        // owned::NbtCompound as a borrow::NbtCompound

        let mut map = HashMap::new();

        for PackedRegistryEntry { id, data } in self.map.get(name)? {
            if let Some(data) = data {
                // convert the value to T
                let mut nbt_bytes = Vec::new();
                data.write(&mut nbt_bytes);
                let nbt_borrow_compound =
                    simdnbt::borrow::NbtCompound::read(&mut Cursor::new(&nbt_bytes)).ok()?;
                let value = match T::from_compound(&nbt_borrow_compound) {
                    Ok(value) => value,
                    Err(err) => {
                        return Some(Err(err));
                    }
                };

                map.insert(id.clone(), value);
            }
        }

        Some(Ok(RegistryType { map }))
    }

    /// Get the dimension type registry, or `None` if it doesn't exist. You
    /// should do some type of error handling if this returns `None`.
    pub fn dimension_type(&self) -> Option<RegistryType<DimensionTypeElement>> {
        let name = ResourceLocation::new("minecraft:dimension_type");
        match self.get(&name) {
            Some(Ok(registry)) => Some(registry),
            Some(Err(err)) => {
                error!(
                    "Error deserializing dimension type registry: {err:?}\n{:?}",
                    self.map.get(&name)
                );
                None
            }
            None => None,
        }
    }
}

/// A collection of values for a certain type of registry data.
#[derive(Debug, Clone)]
pub struct RegistryType<T> {
    pub map: HashMap<ResourceLocation, T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct TrimMaterialElement {
    pub asset_name: String,
    pub ingredient: ResourceLocation,
    pub item_model_index: f32,
    pub override_armor_materials: HashMap<String, String>,
    pub description: Option<String>,
}

/// Data about a kind of chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct ChatTypeElement {
    pub chat: ChatTypeData,
    pub narration: ChatTypeData,
}

/// Data about a chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct ChatTypeData {
    pub translation_key: String,
    pub parameters: Vec<String>,
    pub style: Option<ChatTypeStyle>,
}

/// The style of a chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct ChatTypeStyle {
    pub color: Option<String>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
}

/// Dimension attributes.
#[cfg(feature = "strict_registry")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[simdnbt(deny_unknown_fields)]
pub struct DimensionTypeElement {
    pub ambient_light: f32,
    #[serde(with = "Convert")]
    pub bed_works: bool,
    pub coordinate_scale: f32,
    pub effects: ResourceLocation,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_time: Option<u32>,
    #[serde(with = "Convert")]
    pub has_ceiling: bool,
    #[serde(with = "Convert")]
    pub has_raids: bool,
    #[serde(with = "Convert")]
    pub has_skylight: bool,
    pub height: u32,
    pub infiniburn: ResourceLocation,
    pub logical_height: u32,
    pub min_y: i32,
    pub monster_spawn_block_light_limit: u32,
    pub monster_spawn_light_level: MonsterSpawnLightLevel,
    #[serde(with = "Convert")]
    pub natural: bool,
    #[serde(with = "Convert")]
    pub piglin_safe: bool,
    #[serde(with = "Convert")]
    pub respawn_anchor_works: bool,
    #[serde(with = "Convert")]
    pub ultrawarm: bool,
}

/// Dimension attributes.
#[cfg(not(feature = "strict_registry"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionTypeElement {
    pub height: u32,
    pub min_y: i32,
    #[simdnbt(flatten)]
    pub _extra: HashMap<String, NbtTag>,
}

/// The light level at which monsters can spawn.
///
/// This can be either a single minimum value, or a formula with a min and
/// max.
#[derive(Debug, Clone)]
// #[serde(untagged)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub enum MonsterSpawnLightLevel {
    /// A simple minimum value.
    Simple(u32),
    /// A complex value with a type, minimum, and maximum.
    /// Vanilla minecraft only uses one type, "minecraft:uniform".
    Complex {
        kind: ResourceLocation,
        value: MonsterSpawnLightLevelValues,
    },
}

impl FromNbtTag for MonsterSpawnLightLevel {
    fn from_nbt_tag(tag: &simdnbt::borrow::NbtTag) -> Option<Self> {
        if let Some(value) = tag.int() {
            Some(Self::Simple(value as u32))
        } else if let Some(value) = tag.compound() {
            let kind = ResourceLocation::from_nbt_tag(value.get("type")?)?;
            let value = MonsterSpawnLightLevelValues::from_nbt_tag(value.get("value")?)?;
            Some(Self::Complex { kind, value })
        } else {
            None
        }
    }
}

impl ToNbtTag for MonsterSpawnLightLevel {
    fn to_nbt_tag(self) -> simdnbt::owned::NbtTag {
        match self {
            Self::Simple(value) => value.to_nbt_tag(),
            Self::Complex { kind, value } => {
                let mut compound = NbtCompound::new();
                compound.insert("type", kind.to_nbt_tag());
                compound.insert("value", value.to_nbt_tag());
                simdnbt::owned::NbtTag::Compound(compound)
            }
        }
    }
}

/// The min and max light levels at which monsters can spawn.
///
/// Values are inclusive.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct MonsterSpawnLightLevelValues {
    #[simdnbt(rename = "min_inclusive")]
    pub min: u32,
    #[simdnbt(rename = "max_inclusive")]
    pub max: u32,
}

/// Biome attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct WorldTypeElement {
    pub has_precipitation: bool,
    pub temperature: f32,
    pub temperature_modifier: Option<String>,
    pub downfall: f32,
    pub effects: BiomeEffects,
}

/// The precipitation of a biome.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BiomePrecipitation {
    None,
    Rain,
    Snow,
}
impl FromNbtTag for BiomePrecipitation {
    fn from_nbt_tag(tag: &simdnbt::borrow::NbtTag) -> Option<Self> {
        match tag.string()?.to_str().as_ref() {
            "none" => Some(Self::None),
            "rain" => Some(Self::Rain),
            "snow" => Some(Self::Snow),
            _ => None,
        }
    }
}
impl ToNbtTag for BiomePrecipitation {
    fn to_nbt_tag(self) -> NbtTag {
        match self {
            Self::None => NbtTag::String("none".into()),
            Self::Rain => NbtTag::String("rain".into()),
            Self::Snow => NbtTag::String("snow".into()),
        }
    }
}

/// The effects of a biome.
///
/// This includes the sky, fog, water, and grass color,
/// as well as music and other sound effects.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct BiomeEffects {
    pub sky_color: u32,
    pub fog_color: u32,
    pub water_color: u32,
    pub water_fog_color: u32,
    pub foliage_color: Option<u32>,
    pub grass_color: Option<u32>,
    pub grass_color_modifier: Option<String>,
    pub music: Option<BiomeMusic>,
    pub mood_sound: BiomeMoodSound,
    pub additions_sound: Option<AdditionsSound>,
    pub ambient_sound: Option<ResourceLocation>,
    pub particle: Option<BiomeParticle>,
}

/// The music of the biome.
///
/// Some biomes have unique music that only play when inside them.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct BiomeMusic {
    pub replace_current_music: bool,
    pub max_delay: u32,
    pub min_delay: u32,
    pub sound: azalea_registry::SoundEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct BiomeMoodSound {
    pub tick_delay: u32,
    pub block_search_extent: u32,
    pub offset: f32,
    pub sound: azalea_registry::SoundEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct AdditionsSound {
    pub tick_chance: f32,
    pub sound: azalea_registry::SoundEvent,
}

/// Biome particles.
///
/// Some biomes have particles that spawn in the air.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct BiomeParticle {
    pub probability: f32,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct TrimPatternElement {
    #[simdnbt(flatten)]
    pub pattern: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "strict_registry", simdnbt(deny_unknown_fields))]
pub struct DamageTypeElement {
    pub message_id: String,
    pub scaling: String,
    pub exhaustion: f32,
    pub effects: Option<String>,
    pub death_message_type: Option<String>,
}
