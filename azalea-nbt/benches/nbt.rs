use azalea_nbt::NbtTag;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use flate2::read::GzDecoder;
use std::{
    fs::File,
    io::{Cursor, Read},
};

fn bench_file(filename: &str, c: &mut Criterion) {
    let mut file = File::open(filename).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let mut src = &contents[..];

    // decode the original src so most of the time isn't spent on unzipping
    let mut decoded_src_decoder = GzDecoder::new(&mut src);
    let mut decoded_src = Vec::new();
    if decoded_src_decoder.read_to_end(&mut decoded_src).is_err() {
        // oh probably wasn't gzipped then
        decoded_src = contents;
    }

    let mut decoded_src_stream = Cursor::new(&decoded_src[..]);

    let nbt = NbtTag::read(&mut decoded_src_stream).unwrap();
    decoded_src_stream.set_position(0);

    let mut group = c.benchmark_group(filename);

    group.throughput(Throughput::Bytes(decoded_src.len() as u64));

    // group.bench_function("Decode", |b| {
    //     b.iter(|| {
    //         black_box(Nbt::read(&mut decoded_src_stream).unwrap());
    //         decoded_src_stream.set_position(0);
    //     })
    // });

    // group.bench_function("Encode", |b| {
    //     b.iter(|| {
    //         nbt.write(&mut black_box(Vec::new()));
    //     })
    // });

    group.bench_function("Get", |b| {
        b.iter(|| {
            let level = nbt
                .as_compound()
                .unwrap()
                .get("Level")
                .unwrap()
                .as_compound()
                .unwrap();
            for (k, _) in level.iter() {
                black_box(level.get(black_box(k)));
            }
        })
    });
    group.finish();
}

fn bench(c: &mut Criterion) {
    bench_file("tests/bigtest.nbt", c);
    // bench_file("tests/simple_player.dat", c);
    // bench_file("tests/complex_player.dat", c);
    // bench_file("tests/level.dat", c);
    // bench_file("tests/stringtest.nbt", c);
    // bench_file("tests/inttest16.nbt", c);

    // bench_file("tests/inttest1023.nbt", c);
    // bench_file("tests/inttest3.nbt", c);
}

criterion_group!(benches, bench);
criterion_main!(benches);
