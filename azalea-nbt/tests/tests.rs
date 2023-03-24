use azalea_nbt::{NbtCompound, NbtList, NbtTag};
use std::io::Cursor;

#[test]
fn test_decode_hello_world() {
    // read hello_world.nbt
    let buf = include_bytes!("hello_world.nbt").to_vec();
    let tag = NbtTag::read(&mut Cursor::new(&buf[..])).unwrap();
    assert_eq!(
        tag,
        NbtTag::Compound(NbtCompound::from_iter(vec![(
            "hello world".into(),
            NbtTag::Compound(NbtCompound::from_iter(vec![(
                "name".into(),
                NbtTag::String("Bananrama".into()),
            )]))
        )]))
    );
}

#[test]
fn test_roundtrip_hello_world() {
    let original = include_bytes!("hello_world.nbt").to_vec();

    let mut original_stream = Cursor::new(&original[..]);
    let tag = NbtTag::read(&mut original_stream).unwrap();

    // write hello_world.nbt
    let mut result = Vec::new();
    tag.write(&mut result);

    assert_eq!(result, original);
}

#[test]
fn test_bigtest() {
    // read bigtest.nbt
    let original = include_bytes!("bigtest.nbt").to_vec();

    let mut original_stream = Cursor::new(original);
    let original_tag = NbtTag::read_gzip(&mut original_stream).unwrap();

    let mut result = Vec::new();
    original_tag.write(&mut result);

    let decoded_tag = NbtTag::read(&mut Cursor::new(&result)).unwrap();

    assert_eq!(decoded_tag, original_tag);
}

#[test]
fn test_stringtest() {
    let correct_tag = NbtTag::Compound(NbtCompound::from_iter(vec![(
        "😃".into(),
        NbtTag::List(NbtList::String(vec![
            "asdfkghasfjgihsdfogjsndfg".into(),
            "jnabsfdgihsabguiqwrntgretqwejirhbiqw".into(),
            "asd".into(),
            "wqierjgt7wqy8u4rtbwreithwretiwerutbwenryq8uwervqwer9iuqwbrgyuqrbtwierotugqewrtqwropethert".into(),
            "asdf".into(),
            "alsdkjiqwoe".into(),
            "lmqi9hyqd".into(),
            "qwertyuiop".into(),
            "asdfghjkl".into(),
            "zxcvbnm".into(),
            "                               ".into(),
            "words words words words words words".into(),
            "aaaaaaaaaaaaaaaaaaaa".into(),
            "♥".into(),
            "a\nb\n\n\nc\r\rd".into(),
            "😁".into(),
        ]))
    )]));
    let original = include_bytes!("stringtest.nbt").to_vec();

    let mut original_stream = Cursor::new(original);
    let original_tag = NbtTag::read_gzip(&mut original_stream).unwrap();

    assert_eq!(original_tag, correct_tag);
}

#[test]
fn test_complex_player() {
    let original = include_bytes!("complex_player.dat").to_vec();

    let mut original_stream = Cursor::new(original);
    let original_tag = NbtTag::read_gzip(&mut original_stream).unwrap();

    let mut result = Vec::new();
    original_tag.write(&mut result);

    let decoded_tag = NbtTag::read(&mut Cursor::new(&result)).unwrap();

    assert_eq!(decoded_tag, original_tag);
}

#[test]
fn test_simple_player() {
    let original = include_bytes!("simple_player.dat").to_vec();

    let mut original_stream = Cursor::new(original);
    let original_tag = NbtTag::read_gzip(&mut original_stream).unwrap();

    let mut result = Vec::new();
    original_tag.write(&mut result);

    let decoded_tag = NbtTag::read(&mut Cursor::new(&result)).unwrap();

    assert_eq!(decoded_tag, original_tag);
}

// #[test]
// fn test_inttest() {
//     let original = include_bytes!("inttest.nbt").to_vec();

//     let mut original_stream = Cursor::new(original);
//     let original_tag = Nbt::read_gzip(&mut original_stream).unwrap();

//     let mut result = Vec::new();
//     original_tag.write(&mut result);

//     let decoded_tag = Nbt::read(&mut Cursor::new(&result)).unwrap();

//     assert_eq!(decoded_tag, original_tag);
// }

#[test]
fn test_inttest1023() {
    let original = include_bytes!("inttest1023.nbt").to_vec();

    let mut original_stream = Cursor::new(original.as_slice());
    let original_tag = NbtTag::read(&mut original_stream).unwrap();

    let mut result = Vec::new();
    original_tag.write(&mut result);

    let decoded_tag = NbtTag::read(&mut Cursor::new(&result)).unwrap();

    assert_eq!(decoded_tag, original_tag);
}
