use std::{
    fs::File,
    io::{Cursor, Read},
};

use azalea_buf::McBufReadable;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use flate2::read::GzDecoder;

pub fn bench_read_file(filename: &str, c: &mut Criterion) {
    let mut file = File::open(filename).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let mut src = &contents[..];

    // decode the original src so most of the time isn't spent on unzipping
    let mut decoded_src_decoder = GzDecoder::new(&mut src);
    let mut input = Vec::new();
    decoded_src_decoder.read_to_end(&mut input).unwrap();
    let input = input.as_slice();

    let mut group = c.benchmark_group(filename);
    group.throughput(Throughput::Bytes(input.len() as u64));

    // group.bench_function("azalea_parse", |b| {
    //     b.iter(|| {
    //         let input = black_box(input);
    //         let nbt = azalea_nbt::Nbt::read(&mut Cursor::new(&input)).unwrap();
    //         black_box(nbt);
    //     })
    // });

    // group.bench_function("graphite_parse", |b| {
    //     b.iter(|| {
    //         let input = black_box(input);
    //         let nbt = graphite_binary::nbt::decode::read(&mut
    // &input[..]).unwrap();         black_box(nbt);
    //     })
    // });

    // group.bench_function("valence_parse", |b| {
    //     b.iter(|| {
    //         let input = black_box(input);
    //         let nbt = valence_nbt::from_binary_slice(&mut &input[..]).unwrap();
    //         black_box(nbt);
    //     })
    // });

    // // writing

    let nbt = azalea_nbt::NbtTag::read_from(&mut Cursor::new(input)).unwrap();
    group.bench_function("azalea_write", |b| {
        b.iter(|| {
            let nbt = black_box(&nbt);
            let mut written = Vec::new();
            nbt.write(&mut written);
            black_box(written);
        })
    });

    let nbt = graphite_binary::nbt::decode::read(&mut &input[..]).unwrap();
    group.bench_function("graphite_write", |b| {
        b.iter(|| {
            let nbt = black_box(&nbt);
            let written = graphite_binary::nbt::encode::write(nbt);
            black_box(written);
        })
    });

    // let nbt = valence_nbt::from_binary_slice(&mut &input[..]).unwrap();
    // group.bench_function("valence_write", |b| {
    //     b.iter(|| {
    //         let nbt = black_box(&nbt);
    //         let mut written = Vec::new();
    //         valence_nbt::to_binary_writer(&mut written, &nbt.0,
    // &nbt.1).unwrap();         black_box(written);
    //     })
    // });
}

fn bench(c: &mut Criterion) {
    // bench_read_file("tests/bigtest.nbt", c);
    // bench_read_file("tests/simple_player.dat", c);
    // bench_read_file("tests/complex_player.dat", c);
    // bench_read_file("tests/level.dat", c);
    // bench_read_file("tests/stringtest.nbt", c);
    // bench_read_file("tests/inttest.nbt", c);
}

criterion_group!(benches, bench);
criterion_main!(benches);
