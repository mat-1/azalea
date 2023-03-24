#![doc = include_str!("../README.md")]
#![feature(min_specialization)]

mod decode;
mod encode;
mod error;
mod tag;

pub use error::Error;
pub use tag::{NbtCompound, NbtList};

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::tag::NbtCompound;

    use super::*;
    use azalea_buf::{McBufReadable, McBufWritable};

    #[test]
    fn mcbuf_nbt() {
        let mut buf = Vec::new();
        let tag = NbtTag::Compound(NbtCompound::from_iter(vec![(
            "hello world".into(),
            NbtTag::Compound(NbtCompound::from_iter(vec![(
                "name".into(),
                NbtTag::String("Bananrama".into()),
            )])),
        )]));
        tag.write_into(&mut buf).unwrap();

        let mut buf = Cursor::new(&buf[..]);

        let result = NbtTag::read_from(&mut buf).unwrap();
        assert_eq!(
            result,
            NbtTag::Compound(NbtCompound::from_iter(vec![(
                "hello world".into(),
                NbtTag::Compound(NbtCompound::from_iter(vec![(
                    "name".into(),
                    NbtTag::String("Bananrama".into()),
                )])),
            )]))
        );
    }
}
