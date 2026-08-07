//! Tests for the encoders

mod common;

use image::DynamicImage;

use pica_convert::pica_texture::encode::encode_texture;

#[test]
fn encode_matches_known_good() {
    for format in common::all_formats() {
        for &(width, height) in &common::SIZES {
            let img = DynamicImage::ImageRgba8(common::test_image(width, height));

            let encoded = encode_texture(&img, &format).unwrap();
            let bytes = encoded.data().to_vec();

            let encoded_again = encode_texture(&img, &format).unwrap();
            assert_eq!(
                bytes,
                encoded_again.data(),
                "encode not deterministic for {format:?} {width}x{height}"
            );

            common::assert_known_good("encode", &format, width, height, &bytes);
        }
    }
}
