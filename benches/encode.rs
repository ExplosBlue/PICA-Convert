use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image::RgbaImage;

use pica_convert::pica_texture::encode::{
    encode_a4, encode_a8, encode_etc1, encode_hl8, encode_l4, encode_l8, encode_la44, encode_la88,
    encode_rgb565, encode_rgb888, encode_rgba4444, encode_rgba5551, encode_rgba8888,
};

fn make_img(size: u32) -> RgbaImage {
    RgbaImage::from_fn(size, size, |x, y| {
        image::Rgba([(x * 7) as u8, (y * 11) as u8, (x ^ y) as u8, (x | y) as u8])
    })
}

fn bench_encode(c: &mut Criterion) {
    let img = make_img(512);

    let mut group = c.benchmark_group("encode 512x512");
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(2));
    group.warm_up_time(Duration::from_millis(500));
    group.bench_function("rgba8888", |b| b.iter(|| encode_rgba8888(black_box(&img), 512, 512)));
    group.bench_function("rgb888", |b| b.iter(|| encode_rgb888(black_box(&img), 512, 512)));
    group.bench_function("rgba5551", |b| b.iter(|| encode_rgba5551(black_box(&img), 512, 512)));
    group.bench_function("rgb565", |b| b.iter(|| encode_rgb565(black_box(&img), 512, 512)));
    group.bench_function("rgba4444", |b| b.iter(|| encode_rgba4444(black_box(&img), 512, 512)));
    group.bench_function("la88", |b| b.iter(|| encode_la88(black_box(&img), 512, 512)));
    group.bench_function("hl8", |b| b.iter(|| encode_hl8(black_box(&img), 512, 512)));
    group.bench_function("l8", |b| b.iter(|| encode_l8(black_box(&img), 512, 512)));
    group.bench_function("a8", |b| b.iter(|| encode_a8(black_box(&img), 512, 512)));
    group.bench_function("la44", |b| b.iter(|| encode_la44(black_box(&img), 512, 512)));
    group.bench_function("l4", |b| b.iter(|| encode_l4(black_box(&img), 512, 512)));
    group.bench_function("a4", |b| b.iter(|| encode_a4(black_box(&img), 512, 512)));
    group.finish();

    let mut group = c.benchmark_group("encode_etc1 512x512");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(30));
    group.warm_up_time(Duration::from_millis(1000));
    group.bench_function("no_alpha", |b| {
        b.iter(|| encode_etc1(black_box(&img), 512, 512, black_box(false)))
    });
    group.bench_function("alpha", |b| {
        b.iter(|| encode_etc1(black_box(&img), 512, 512, black_box(true)))
    });
    group.finish();
}

criterion_group!(benches, bench_encode);
criterion_main!(benches);
