use crate::tag::*;
use azalea_buf::McBufWritable;
use byteorder::{WriteBytesExt, BE};
use flate2::write::{GzEncoder, ZlibEncoder};
use packed_simd_2::{i32x16, i32x2, i32x4, i32x8, i64x2, i64x4, i64x8};
use std::io::Write;

#[inline(always)]
fn write_string(writer: &mut impl Write, string: &NbtString) {
    writer.write_u16::<BE>(string.len() as u16).unwrap();
    writer.write_all(string.as_bytes()).unwrap();
}

#[inline(always)]
fn write_compound(writer: &mut impl Write, value: &NbtCompound, end_tag: bool) {
    for (key, tag) in value.iter() {
        writer.write_u8(tag.id()).unwrap();
        write_string(writer, key);
        match tag {
            UntypedNbtTag::End => {}
            NbtTag::Byte(value) => {
                writer.write_i8(*value).unwrap();
            }
            NbtTag::Short(value) => {
                writer.write_i16::<BE>(*value).unwrap();
            }
            NbtTag::Int(value) => {
                writer.write_i32::<BE>(*value).unwrap();
            }
            NbtTag::Long(value) => {
                writer.write_i64::<BE>(*value).unwrap();
            }
            NbtTag::Float(value) => {
                writer.write_f32::<BE>(*value).unwrap();
            }
            NbtTag::Double(value) => {
                writer.write_f64::<BE>(*value).unwrap();
            }
            NbtTag::ByteArray(value) => {
                write_byte_array(writer, value);
            }
            NbtTag::String(value) => {
                write_string(writer, value);
            }
            NbtTag::List(value) => {
                write_list(writer, value);
            }
            NbtTag::Compound(value) => {
                write_compound(writer, value, true);
            }
            NbtTag::IntArray(value) => {
                write_int_array(writer, value);
            }
            NbtTag::LongArray(value) => {
                write_long_array(writer, value);
            }
        }
    }
    if end_tag {
        writer.write_u8(END_ID).unwrap();
    }
}

#[inline(always)]
fn write_list(writer: &mut impl Write, value: &NbtList) {
    writer.write_u8(value.id()).unwrap();
    match value {
        NbtList::Empty => writer.write_all(&[0; 4]).unwrap(),
        NbtList::Byte(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            let l = l.as_slice();
            writer
                // convert [i8] into [u8]
                .write_all(unsafe { std::slice::from_raw_parts(l.as_ptr() as *const u8, l.len()) })
                .unwrap();
        }
        NbtList::Short(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for &v in l {
                writer.write_i16::<BE>(v).unwrap();
            }
        }
        NbtList::Int(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            // flip the bits to big endian with simd
            let l = l.as_slice();
            let mut position = 0;
            // x16
            while l.len() - position >= 16 {
                let l = unsafe { i32x16::from_slice_unaligned_unchecked(&l[position..]) };
                l.to_be();
                let l = unsafe { std::mem::transmute::<i32x16, [u8; 64]>(l) };
                writer.write_all(&l).unwrap();
                position += 16;
            }
            // x8
            if l.len() - position >= 8 {
                let l = unsafe { i32x8::from_slice_unaligned_unchecked(&l[position..]) };
                l.to_be();
                let l = unsafe { std::mem::transmute::<i32x8, [u8; 32]>(l) };
                writer.write_all(&l).unwrap();
                position += 8;
            }
            // x4
            if l.len() - position >= 4 {
                let l = unsafe { i32x4::from_slice_unaligned_unchecked(&l[position..]) };
                l.to_be();
                let l = unsafe { std::mem::transmute::<i32x4, [u8; 16]>(l) };
                writer.write_all(&l).unwrap();
                position += 4;
            }
            // x2
            if l.len() - position >= 2 {
                let l = unsafe { i32x2::from_slice_unaligned_unchecked(&l[position..]) };
                l.to_be();
                let l = unsafe { std::mem::transmute::<i32x2, [u8; 8]>(l) };
                writer.write_all(&l).unwrap();
                position += 2;
            }
            // x1 ... just a normal write_i32
            if l.len() - position >= 1 {
                writer.write_i32::<BE>(l[position]).unwrap();
            }
        }
        NbtList::Long(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            // flip the bits to big endian with simd
            let l = l.as_slice();
            let mut position = 0;
            // x16
            while l.len() - position >= 8 {
                let l = unsafe { i64x8::from_slice_unaligned_unchecked(&l[position..]) };
                l.to_be();
                let l = unsafe { std::mem::transmute::<i64x8, [u8; 64]>(l) };
                writer.write_all(&l).unwrap();
                position += 8;
            }
            // x4
            if l.len() - position >= 4 {
                let l = unsafe { i64x4::from_slice_unaligned_unchecked(&l[position..]) };
                l.to_be();
                let l = unsafe { std::mem::transmute::<i64x4, [u8; 32]>(l) };
                writer.write_all(&l).unwrap();
                position += 4;
            }
            // x2
            if l.len() - position >= 2 {
                let l = unsafe { i64x2::from_slice_unaligned_unchecked(&l[position..]) };
                l.to_be();
                let l = unsafe { std::mem::transmute::<i64x2, [u8; 16]>(l) };
                writer.write_all(&l).unwrap();
                position += 2;
            }
            // x1 ... just a normal write_i32
            if l.len() - position >= 1 {
                writer.write_i64::<BE>(l[position]).unwrap();
            }
        }
        NbtList::Float(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for &v in l {
                writer.write_f32::<BE>(v).unwrap();
            }
        }
        NbtList::Double(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for &v in l {
                writer.write_f64::<BE>(v).unwrap();
            }
        }
        NbtList::ByteArray(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for v in l {
                write_byte_array(writer, v);
            }
        }
        NbtList::String(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for v in l {
                write_string(writer, v);
            }
        }
        NbtList::List(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for v in l {
                write_list(writer, v);
            }
        }
        NbtList::Compound(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for v in l {
                write_compound(writer, v, true);
            }
        }
        NbtList::IntArray(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for v in l {
                write_int_array(writer, v);
            }
        }
        NbtList::LongArray(l) => {
            writer.write_i32::<BE>(l.len() as i32).unwrap();
            for v in l {
                write_long_array(writer, v);
            }
        }
    }
}

#[inline]
fn write_byte_array(writer: &mut impl Write, value: &[u8]) {
    writer.write_u32::<BE>(value.len() as u32).unwrap();
    writer.write_all(value).unwrap();
}

#[inline]
fn write_int_array(writer: &mut impl Write, value: &Vec<i32>) {
    writer.write_u32::<BE>(value.len() as u32).unwrap();
    for &int in value {
        writer.write_i32::<BE>(int).unwrap();
    }
}

#[inline]
fn write_long_array(writer: &mut impl Write, value: &Vec<i64>) {
    writer.write_u32::<BE>(value.len() as u32).unwrap();
    for &long in value {
        writer.write_i64::<BE>(long).unwrap();
    }
}

impl NbtTag {
    /// Write the compound tag as NBT data.
    ///
    /// # Panics
    ///
    /// Will panic if the tag is not a Compound or End tag.
    pub fn write(&self, writer: &mut impl Write) {
        match self {
            NbtTag::Compound(value) => {
                write_compound(writer, value, false);
            }
            NbtTag::End => {
                END_ID.write_into(writer).unwrap();
            }
            _ => panic!("Not a compound tag"),
        }
    }

    /// Write the compound tag as NBT data compressed wtih zlib.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if it's not a Compound or End tag.
    pub fn write_zlib(&self, writer: &mut impl Write) {
        let mut encoder = ZlibEncoder::new(writer, flate2::Compression::default());
        self.write(&mut encoder)
    }

    /// Write the compound tag as NBT data compressed wtih gzip.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if it's not a Compound or End tag.
    pub fn write_gzip(&self, writer: &mut impl Write) {
        let mut encoder = GzEncoder::new(writer, flate2::Compression::default());
        self.write(&mut encoder)
    }
}

impl McBufWritable for NbtTag {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.write(buf);
        Ok(())
    }
}
