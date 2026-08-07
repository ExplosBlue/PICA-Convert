use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image::RgbaImage;

use pica_convert::pica_texture::decode::{
    decode_a4, decode_a8, decode_etc1, decode_hl8, decode_l4, decode_l8, decode_la44, decode_la88,
    decode_texture,
};
use pica_convert::pica_texture::encode::{
    encode_a4, encode_a8, encode_etc1, encode_hl8, encode_l4, encode_l8, encode_la44, encode_la88,
};
use pica_convert::pica_texture::types::{PicaTexture, TextureFormat};

fn make_img(size: u32) -> RgbaImage {
    RgbaImage::from_fn(size, size, |x, y| {
        image::Rgba([(x * 7) as u8, (y * 11) as u8, (x ^ y) as u8, (x | y) as u8])
    })
}

fn bench_decode(c: &mut Criterion) {
    let img = make_img(512);

    let la88 = encode_la88(&img, 512, 512);
    let hl8 = encode_hl8(&img, 512, 512);
    let l8 = encode_l8(&img, 512, 512);
    let a8 = encode_a8(&img, 512, 512);
    let la44 = encode_la44(&img, 512, 512);
    let l4 = encode_l4(&img, 512, 512);
    let a4 = encode_a4(&img, 512, 512);
    let etc1 = encode_etc1(&img, 512, 512, false);
    let etc1a4 = encode_etc1(&img, 512, 512, true);

    let mut group = c.benchmark_group("decode 512x512");
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(2));
    group.warm_up_time(Duration::from_millis(500));
    group.bench_function("la88", |b| b.iter(|| decode_la88(black_box(&la88), 512, 512)));
    group.bench_function("hl8", |b| b.iter(|| decode_hl8(black_box(&hl8), 512, 512)));
    group.bench_function("l8", |b| b.iter(|| decode_l8(black_box(&l8), 512, 512)));
    group.bench_function("a8", |b| b.iter(|| decode_a8(black_box(&a8), 512, 512)));
    group.bench_function("la44", |b| b.iter(|| decode_la44(black_box(&la44), 512, 512)));
    group.bench_function("l4", |b| b.iter(|| decode_l4(black_box(&l4), 512, 512)));
    group.bench_function("a4", |b| b.iter(|| decode_a4(black_box(&a4), 512, 512)));
    group.bench_function("etc1", |b| b.iter(|| decode_etc1(black_box(&etc1), 512, 512, false)));
    group.bench_function("etc1a4", |b| b.iter(|| decode_etc1(black_box(&etc1a4), 512, 512, true)));
    group.finish();

    let tex_la88 = PicaTexture::new(TextureFormat::LA88, 512, 512, la88);
    let tex_etc1 = PicaTexture::new(TextureFormat::ETC1, 512, 512, etc1);

    let mut group = c.benchmark_group("decode_texture 512x512");
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(2));
    group.warm_up_time(Duration::from_millis(500));
    group.bench_function("la88", |b| b.iter(|| decode_texture(black_box(&tex_la88))));
    group.bench_function("etc1", |b| b.iter(|| decode_texture(black_box(&tex_etc1))));
    group.finish();
}

criterion_group!(benches, bench_decode);
criterion_main!(benches);
