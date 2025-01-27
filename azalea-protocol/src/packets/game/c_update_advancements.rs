use std::collections::HashMap;
use std::io::Cursor;

use azalea_buf::AzBuf;
use azalea_chat::FormattedText;
use azalea_core::resource_location::ResourceLocation;
use azalea_inventory::ItemStack;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(Clone, Debug, AzBuf, ClientboundGamePacket)]
pub struct ClientboundUpdateAdvancements {
    pub reset: bool,
    pub added: Vec<AdvancementHolder>,
    pub removed: Vec<ResourceLocation>,
    pub progress: HashMap<ResourceLocation, AdvancementProgress>,
    pub show_advancements: bool,
}

#[derive(Clone, Debug, AzBuf)]
pub struct Advancement {
    pub parent_id: Option<ResourceLocation>,
    pub display: Option<DisplayInfo>,
    pub requirements: Vec<Vec<String>>,
    pub sends_telemetry_event: bool,
}

#[derive(Clone, Debug)]
pub struct DisplayInfo {
    pub title: FormattedText,
    pub description: FormattedText,
    pub icon: ItemStack,
    pub frame: FrameType,
    pub show_toast: bool,
    pub hidden: bool,
    pub background: Option<ResourceLocation>,
    pub x: f32,
    pub y: f32,
}

impl azalea_buf::AzaleaWrite for DisplayInfo {
    fn azalea_write(&self, buf: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        self.title.azalea_write(buf)?;
        self.description.azalea_write(buf)?;
        self.icon.azalea_write(buf)?;
        self.frame.azalea_write(buf)?;

        let mut data: u32 = 0;
        if self.background.is_some() {
            data |= 0b001;
        }
        if self.show_toast {
            data |= 0b010;
        }
        if self.hidden {
            data |= 0b100;
        }
        data.azalea_write(buf)?;

        if let Some(background) = &self.background {
            background.azalea_write(buf)?;
        }
        self.x.azalea_write(buf)?;
        self.y.azalea_write(buf)?;
        Ok(())
    }
}
impl azalea_buf::AzaleaRead for DisplayInfo {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, azalea_buf::BufReadError> {
        let title = azalea_buf::AzaleaRead::azalea_read(buf)?;
        let description = azalea_buf::AzaleaRead::azalea_read(buf)?;
        let icon = azalea_buf::AzaleaRead::azalea_read(buf)?;
        let frame = azalea_buf::AzaleaRead::azalea_read(buf)?;

        let data = u32::azalea_read(buf)?;
        let has_background = (data & 0b1) != 0;
        let show_toast = (data & 0b10) != 0;
        let hidden = (data & 0b100) != 0;

        let background = if has_background {
            Some(ResourceLocation::azalea_read(buf)?)
        } else {
            None
        };
        let x = azalea_buf::AzaleaRead::azalea_read(buf)?;
        let y = azalea_buf::AzaleaRead::azalea_read(buf)?;
        Ok(DisplayInfo {
            title,
            description,
            icon,
            frame,
            show_toast,
            hidden,
            background,
            x,
            y,
        })
    }
}

#[derive(Clone, Debug, Copy, AzBuf)]
pub enum FrameType {
    Task = 0,
    Challenge = 1,
    Goal = 2,
}

pub type AdvancementProgress = HashMap<String, CriterionProgress>;

#[derive(Clone, Debug, AzBuf)]
pub struct CriterionProgress {
    pub date: Option<u64>,
}

#[derive(Clone, Debug, AzBuf)]
pub struct AdvancementHolder {
    pub id: ResourceLocation,
    pub value: Advancement,
}

#[cfg(test)]
mod tests {
    use azalea_buf::{AzaleaRead, AzaleaWrite};

    use super::*;

    #[test]
    fn test() {
        let packet = ClientboundUpdateAdvancements {
            reset: true,
            added: [AdvancementHolder {
                id: ResourceLocation::new("minecraft:test"),
                value: Advancement {
                    parent_id: None,
                    display: Some(DisplayInfo {
                        title: FormattedText::from("title".to_string()),
                        description: FormattedText::from("description".to_string()),
                        icon: ItemStack::Empty,
                        frame: FrameType::Task,
                        show_toast: true,
                        hidden: false,
                        background: None,
                        x: 0.0,
                        y: 0.0,
                    }),
                    requirements: Vec::new(),
                    sends_telemetry_event: false,
                },
            }]
            .into_iter()
            .collect(),
            removed: vec![ResourceLocation::new("minecraft:test2")],
            progress: [(
                ResourceLocation::new("minecraft:test3"),
                [(
                    "minecraft:test4".to_string(),
                    CriterionProgress {
                        date: Some(123456789),
                    },
                )]
                .into_iter()
                .collect(),
            )]
            .into_iter()
            .collect(),
        };

        let mut data = Vec::new();
        packet.azalea_write(&mut data).unwrap();
        let mut buf: Cursor<&[u8]> = Cursor::new(&data);

        let read_packet = ClientboundUpdateAdvancements::azalea_read(&mut buf).unwrap();
        assert_eq!(packet.reset, read_packet.reset);
        assert_eq!(packet.removed, read_packet.removed);

        let advancement = packet
            .added
            .into_iter()
            .find_map(|a| {
                if a.id == ResourceLocation::new("minecraft:test") {
                    Some(a.value)
                } else {
                    None
                }
            })
            .unwrap()
            .clone();
        let read_advancement = read_packet
            .added
            .into_iter()
            .find_map(|a| {
                if a.id == ResourceLocation::new("minecraft:test") {
                    Some(a.value)
                } else {
                    None
                }
            })
            .unwrap()
            .clone();
        assert_eq!(advancement.parent_id, read_advancement.parent_id);

        let display = advancement.display.unwrap();
        let read_display = read_advancement.display.unwrap();
        assert_eq!(display.title, read_display.title);
        assert_eq!(display.description, read_display.description);
    }
}
