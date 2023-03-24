// TODO: have an azalea-inventory or azalea-container crate and put this there

use azalea_buf::{BufReadError, McBuf, McBufReadable, McBufWritable};
use azalea_nbt::NbtTag;
use std::io::{Cursor, Write};

#[derive(Debug, Clone, Default)]
pub enum Slot {
    #[default]
    Empty,
    Present(SlotData),
}

#[derive(Debug, Clone, McBuf)]
pub struct SlotData {
    #[var]
    pub id: u32,
    pub count: u8,
    pub nbt: NbtTag,
}

impl McBufReadable for Slot {
    fn read_from(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let slot = Option::<SlotData>::read_from(buf)?;
        Ok(slot.map_or(Slot::Empty, Slot::Present))
    }
}

impl McBufWritable for Slot {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        match self {
            Slot::Empty => false.write_into(buf)?,
            Slot::Present(i) => {
                true.write_into(buf)?;
                i.write_into(buf)?;
            }
        };
        Ok(())
    }
}
