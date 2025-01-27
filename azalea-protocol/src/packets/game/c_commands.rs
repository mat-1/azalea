use std::io::{Cursor, Write};

use azalea_buf::{AzBuf, AzaleaRead, AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};
use azalea_core::{bitset::FixedBitSet, resource_location::ResourceLocation};
use azalea_protocol_macros::ClientboundGamePacket;
use tracing::warn;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundCommands {
    pub entries: Vec<BrigadierNodeStub>,
    #[var]
    pub root_index: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BrigadierNodeStub {
    pub is_executable: bool,
    pub children: Vec<u32>,
    pub redirect_node: Option<u32>,
    pub node_type: NodeType,
}

#[derive(Debug, Clone, Eq)]
pub struct BrigadierNumber<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}
impl<T> BrigadierNumber<T> {
    pub fn new(min: Option<T>, max: Option<T>) -> BrigadierNumber<T> {
        BrigadierNumber { min, max }
    }
}
impl<T: PartialEq> PartialEq for BrigadierNumber<T> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.min, &self.max, &other.min, &other.max) {
            (Some(f_min), None, Some(s_min), None) => f_min == s_min,
            (None, Some(f_max), None, Some(s_max)) => f_max == s_max,
            (Some(f_min), Some(f_max), Some(s_min), Some(s_max)) => {
                f_min == s_min && f_max == s_max
            }
            (None, None, None, None) => true,
            _ => false,
        }
    }
}

impl<T: AzaleaRead> AzaleaRead for BrigadierNumber<T> {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let flags = FixedBitSet::<{ 2_usize.div_ceil(8) }>::azalea_read(buf)?;
        let min = if flags.index(0) {
            Some(T::azalea_read(buf)?)
        } else {
            None
        };
        let max = if flags.index(1) {
            Some(T::azalea_read(buf)?)
        } else {
            None
        };
        Ok(BrigadierNumber { min, max })
    }
}
impl<T: AzaleaWrite> AzaleaWrite for BrigadierNumber<T> {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut flags = FixedBitSet::<{ 2_usize.div_ceil(8) }>::new();
        if self.min.is_some() {
            flags.set(0);
        }
        if self.max.is_some() {
            flags.set(1);
        }
        flags.azalea_write(buf)?;
        if let Some(min) = &self.min {
            min.azalea_write(buf)?;
        }
        if let Some(max) = &self.max {
            max.azalea_write(buf)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, AzBuf, PartialEq, Eq)]
pub enum BrigadierString {
    /// Reads a single word
    SingleWord = 0,
    // If it starts with a ", keeps reading until another " (allowing escaping with \). Otherwise
    // behaves the same as SINGLE_WORD
    QuotablePhrase = 1,
    // Reads the rest of the content after the cursor. Quotes will not be removed.
    GreedyPhrase = 2,
}

// see ArgumentTypeInfo
#[derive(Debug, Clone, AzBuf, PartialEq)]
pub enum BrigadierParser {
    Bool,
    Float(BrigadierNumber<f32>),
    Double(BrigadierNumber<f64>),
    Integer(BrigadierNumber<i32>),
    Long(BrigadierNumber<i64>),
    String(BrigadierString),
    Entity(EntityParser),
    GameProfile,
    BlockPos,
    ColumnPos,
    Vec3,
    Vec2,
    BlockState,
    BlockPredicate,
    ItemStack,
    ItemPredicate,
    Color,
    FormattedText,
    Style,
    Message,
    NbtCompoundTag,
    NbtTag,
    NbtPath,
    Objective,
    ObjectiveCriteria,
    Operation,
    Particle,
    Angle,
    Rotation,
    ScoreboardSlot,
    ScoreHolder { allows_multiple: bool },
    Swizzle,
    Team,
    ItemSlot,
    ItemSlots,
    ResourceLocation,
    Function,
    EntityAnchor,
    IntRange,
    FloatRange,
    Dimension,
    GameMode,
    Time { min: i32 },
    ResourceOrTag { registry_key: ResourceLocation },
    ResourceOrTagKey { registry_key: ResourceLocation },
    Resource { registry_key: ResourceLocation },
    ResourceKey { registry_key: ResourceLocation },
    ResourceSelector { registry_key: ResourceLocation },
    TemplateMirror,
    TemplateRotation,
    Heightmap,
    LootTable,
    LootPredicate,
    LootModifier,
    Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityParser {
    pub single: bool,
    pub players_only: bool,
}
impl AzaleaRead for EntityParser {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let flags = FixedBitSet::<{ 2_usize.div_ceil(8) }>::azalea_read(buf)?;
        Ok(EntityParser {
            single: flags.index(0),
            players_only: flags.index(1),
        })
    }
}
impl AzaleaWrite for EntityParser {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut flags = FixedBitSet::<{ 2_usize.div_ceil(8) }>::new();
        if self.single {
            flags.set(0);
        }
        if self.players_only {
            flags.set(1);
        }
        flags.azalea_write(buf)?;
        Ok(())
    }
}

// TODO: BrigadierNodeStub should have more stuff
impl AzaleaRead for BrigadierNodeStub {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let flags = FixedBitSet::<{ 8_usize.div_ceil(8) }>::azalea_read(buf)?;
        if flags.index(5) || flags.index(6) || flags.index(7) {
            warn!("The flags from a Brigadier node are over 31. This is a bug, BrigadierParser probably needs updating.",);
        }

        let node_type = u8::from(flags.index(0)) + (u8::from(flags.index(1)) * 2);
        let is_executable = flags.index(2);
        let has_redirect = flags.index(3);
        let has_suggestions_type = flags.index(4);

        let children = Vec::<u32>::azalea_read_var(buf)?;
        let redirect_node = if has_redirect {
            Some(u32::azalea_read_var(buf)?)
        } else {
            None
        };

        // argument node
        if node_type == 2 {
            let name = String::azalea_read(buf)?;
            let parser = BrigadierParser::azalea_read(buf)?;
            let suggestions_type = if has_suggestions_type {
                Some(ResourceLocation::azalea_read(buf)?)
            } else {
                None
            };
            let node = BrigadierNodeStub {
                is_executable,
                children,
                redirect_node,
                node_type: NodeType::Argument {
                    name,
                    parser,
                    suggestions_type,
                },
            };
            return Ok(node);
        }
        // literal node
        else if node_type == 1 {
            let name = String::azalea_read(buf)?;
            return Ok(BrigadierNodeStub {
                is_executable,
                children,
                redirect_node,
                node_type: NodeType::Literal { name },
            });
        }
        Ok(BrigadierNodeStub {
            is_executable,
            children,
            redirect_node,
            node_type: NodeType::Root,
        })
    }
}

impl AzaleaWrite for BrigadierNodeStub {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        let mut flags = FixedBitSet::<{ 4_usize.div_ceil(8) }>::new();
        if self.is_executable {
            flags.set(2);
        }
        if self.redirect_node.is_some() {
            flags.set(3);
        }

        match &self.node_type {
            NodeType::Root => {
                flags.azalea_write(buf)?;

                self.children.azalea_write_var(buf)?;

                if let Some(redirect) = self.redirect_node {
                    redirect.azalea_write_var(buf)?;
                }
            }
            NodeType::Literal { name } => {
                flags.set(0);
                flags.azalea_write(buf)?;

                self.children.azalea_write_var(buf)?;

                if let Some(redirect) = self.redirect_node {
                    redirect.azalea_write_var(buf)?;
                }

                name.azalea_write(buf)?;
            }
            NodeType::Argument {
                name,
                parser,
                suggestions_type,
            } => {
                flags.set(1);
                if suggestions_type.is_some() {
                    flags.set(4);
                }
                flags.azalea_write(buf)?;

                self.children.azalea_write_var(buf)?;

                if let Some(redirect) = self.redirect_node {
                    redirect.azalea_write_var(buf)?;
                }

                name.azalea_write(buf)?;
                parser.azalea_write(buf)?;

                if let Some(suggestion) = suggestions_type {
                    suggestion.azalea_write(buf)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Root,
    Literal {
        name: String,
    },
    Argument {
        name: String,
        parser: BrigadierParser,
        suggestions_type: Option<ResourceLocation>,
    },
}

impl BrigadierNodeStub {
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match &self.node_type {
            NodeType::Root => None,
            NodeType::Literal { name } | NodeType::Argument { name, .. } => Some(name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brigadier_node_stub_root() {
        let data = BrigadierNodeStub {
            is_executable: false,
            children: vec![1, 2],
            redirect_node: None,
            node_type: NodeType::Root,
        };
        let mut buf = Vec::new();
        data.azalea_write(&mut buf).unwrap();
        let mut data_cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let read_data = BrigadierNodeStub::azalea_read(&mut data_cursor).unwrap();
        assert_eq!(data, read_data);
    }

    #[test]
    fn test_brigadier_node_stub_literal() {
        let data = BrigadierNodeStub {
            is_executable: true,
            children: vec![],
            redirect_node: None,
            node_type: NodeType::Literal {
                name: "String".to_string(),
            },
        };
        let mut buf = Vec::new();
        data.azalea_write(&mut buf).unwrap();
        let mut data_cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let read_data = BrigadierNodeStub::azalea_read(&mut data_cursor).unwrap();
        assert_eq!(data, read_data);
    }

    #[test]
    fn test_brigadier_node_stub_argument() {
        let data = BrigadierNodeStub {
            is_executable: false,
            children: vec![6, 9],
            redirect_node: Some(5),
            node_type: NodeType::Argument {
                name: "position".to_string(),
                parser: BrigadierParser::Vec3,
                suggestions_type: Some(ResourceLocation::new("minecraft:test_suggestion")),
            },
        };
        let mut buf = Vec::new();
        data.azalea_write(&mut buf).unwrap();
        let mut data_cursor: Cursor<&[u8]> = Cursor::new(&buf);
        let read_data = BrigadierNodeStub::azalea_read(&mut data_cursor).unwrap();
        assert_eq!(data, read_data);
    }
}
