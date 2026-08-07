//! Tests for the decoders

mod common;

use image::DynamicImage;

use pica_convert::pica_texture::{PicaTexture, decode_texture, encode::encode_texture};

#[test]
fn decode_matches_known_good() {
    for format in common::all_formats() {
        for &(width, height) in &common::SIZES {
            let img = DynamicImage::ImageRgba8(common::test_image(width, height));

            let encoded = encode_texture(&img, &format).unwrap();
            let texture = PicaTexture::new(format.clone(), width, height, encoded.data().to_vec());

            let decoded = decode_texture(&texture).unwrap().to_rgba8().into_raw();

            let texture_again =
                PicaTexture::new(format.clone(), width, height, encoded.data().to_vec());

            let decoded_again = decode_texture(&texture_again)
                .unwrap()
                .to_rgba8()
                .into_raw();

            assert_eq!(
                decoded, decoded_again,
                "decode not deterministic for {format:?} {width}x{height}"
            );

            common::assert_known_good("decode", &format, width, height, &decoded);
        }
    }
}
