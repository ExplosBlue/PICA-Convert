//! Shared helpers for testing.
//!
//! Known-good fixtures capture the current known-good output of each
//! encoder/decoder. Set `PICA_REGEN_KNOWN_GOOD=1` when running the tests to
//! rewrite them (e.g. after a deliberate format change), then commit the
//! updated files.

use image::{Rgba, RgbaImage};
use std::path::PathBuf;

use pica_convert::pica_texture::TextureFormat;

pub const SIZES: [(u32, u32); 2] = [(8, 8), (16, 16)];

/// Deterministic test image pattern covering a wide range of pixel values.
pub fn test_image(width: u32, height: u32) -> RgbaImage {
    RgbaImage::from_fn(width, height, |x, y| {
        Rgba([
            (x.wrapping_mul(7)) as u8,
            (y.wrapping_mul(11)) as u8,
            (x ^ y) as u8,
            ((x + y) & 0xFF) as u8,
        ])
    })
}

pub fn all_formats() -> [TextureFormat; 14] {
    [
        TextureFormat::RGBA8888,
        TextureFormat::RGB888,
        TextureFormat::RGBA5551,
        TextureFormat::RGB565,
        TextureFormat::RGBA4444,
        TextureFormat::LA88,
        TextureFormat::HL8,
        TextureFormat::L8,
        TextureFormat::A8,
        TextureFormat::LA44,
        TextureFormat::L4,
        TextureFormat::A4,
        TextureFormat::ETC1,
        TextureFormat::ETC1A4,
    ]
}

pub fn format_name(format: &TextureFormat) -> &'static str {
    match format {
        TextureFormat::RGBA8888 => "rgba8888",
        TextureFormat::RGB888 => "rgb888",
        TextureFormat::RGBA5551 => "rgba5551",
        TextureFormat::RGB565 => "rgb565",
        TextureFormat::RGBA4444 => "rgba4444",
        TextureFormat::LA88 => "la88",
        TextureFormat::HL8 => "hl8",
        TextureFormat::L8 => "l8",
        TextureFormat::A8 => "a8",
        TextureFormat::LA44 => "la44",
        TextureFormat::L4 => "l4",
        TextureFormat::A4 => "a4",
        TextureFormat::ETC1 => "etc1",
        TextureFormat::ETC1A4 => "etc1a4",
    }
}

pub fn known_good_path(kind: &str, format: &TextureFormat, width: u32, height: u32) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("known_good")
        .join(kind)
        .join(format!("{}_{}x{}.bin", format_name(format), width, height))
}

fn should_regenerate() -> bool {
    std::env::var_os("PICA_REGEN_KNOWN_GOOD").is_some()
}

/// Compare `data` against the known-good fixture, or rewrite the fixture when
/// `PICA_REGEN_KNOWN_GOOD=1` is set.
pub fn assert_known_good(
    kind: &str,
    format: &TextureFormat,
    width: u32,
    height: u32,
    data: &[u8],
) {
    let path = known_good_path(kind, format, width, height);

    if should_regenerate() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&path, data).unwrap();
        return;
    }

    let expected = std::fs::read(&path).unwrap_or_else(|e| {
        panic!(
            "missing known-good fixture {}: {e}\n\
             (run with PICA_REGEN_KNOWN_GOOD=1 to regenerate)",
            path.display()
        )
    });

    assert_eq!(
        data,
        &expected[..],
        "{} output differs from known-good fixture {}",
        format_name(format),
        path.display()
    );
}
