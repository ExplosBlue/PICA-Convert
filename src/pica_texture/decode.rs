use image::{DynamicImage, ImageBuffer};

use crate::pica_texture::{TextureFormat, util, types::SWIZZLE_LUT};

/// Decodes raw PICA texture data into a [`DynamicImage`].
///
/// This function takes raw texture bytes along with image dimensions and a
/// [`TextureFormat`] enum, decodes the pixel data into RGBA format, and flips
/// the image vertically to match the expected orientation.
///
/// # Arguments
///
/// * `img` - A byte slice containing the raw texture data.
/// * `width` - The width of the texture in pixels.
/// * `height` - The height of the texture in pixels.
/// * `format` - The [`TextureFormat`] describing how the texture data is encoded.
///
/// # Returns
///
/// * `Ok(DynamicImage)` containing the decoded image if successful.
/// * `Err(Box<dyn std::error::Error>)` if the texture cannot be decoded or the
///   raw data cannot be converted into a valid image buffer.
///
/// # Errors
///
/// Returns an error if:
/// - The provided `img` data is not valid for the given dimensions and format.
/// - Construction of the [`ImageBuffer`] fails (e.g., data length mismatch).
///
/// # Examples
///
/// ```
/// use pica_convert::pica_texture::{decode_texture, TextureFormat};
///
/// // Suppose `raw_bytes` contains valid RGBA8888 texture data.
/// let width = 128;
/// let height = 128;
/// let format = TextureFormat::RGBA8888;
///
/// let decoded = decode_texture(&raw_bytes, width, height, &format).unwrap();
/// assert_eq!(decoded.width(), 128);
/// assert_eq!(decoded.height(), 128);
/// ```
pub fn decode_texture(img: &[u8], width: u32, height: u32, format: &TextureFormat) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    println!("Decoding texture with dimensions {}x{}", width, height);

    let mut decoded_texture_data = match format {
        TextureFormat::RGBA8888 => decode_rgba8888(img, width, height),
        TextureFormat::RGB888   => decode_rgb888(img, width, height),
        TextureFormat::RGBA5551 => decode_rgba5551(img, width, height),
        TextureFormat::RGB565   => decode_rgb565(img, width, height),
        TextureFormat::RGBA4444 => decode_rgba4444(img, width, height),
        _ => unimplemented!("Decoding for the specified format is not implemented yet"),
    };

    // Flip decoded texture vertically
    util::flip_vertical(&mut decoded_texture_data, width, height);

    let decoded_image = ImageBuffer::from_raw(width, height, decoded_texture_data)
        .map(DynamicImage::ImageRgba8)
        .ok_or("Failed to construct ImageBuffer from raw data")?;

    Ok(decoded_image)
}

/// Decodes RGBA8888 PICA texture data into a `Vec<u8>` of RGBA texture data.
///
/// # Arguments
///
/// * `texture_data` - A byte slice containing the raw texture data.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the decoded RGBA data.
///
fn decode_rgba8888(texture_data: &[u8], width: u32, height: u32) -> Vec<u8> {
    println!("Decoding as RGBA8888");

    let bytes_per_pixel = 32 / 8;
    let mut output: Vec<u8> = vec![0; (width * height * 4) as usize];
    let mut src_idx: usize = 0;

    for ty in (0..height).step_by(8) {
        for tx in (0..width).step_by(8) {
            for px in SWIZZLE_LUT {

                let x = px & 7;
                let y = (px - x) >> 3;

                let out_idx = ((tx + x + (height - 1 - (ty + y)) * width) * 4) as usize;

                output[out_idx    ] = texture_data[src_idx + 3];
                output[out_idx + 1] = texture_data[src_idx + 2];
                output[out_idx + 2] = texture_data[src_idx + 1];
                output[out_idx + 3] = texture_data[src_idx    ];

                src_idx += bytes_per_pixel;
            }
        }
    }
    output
}

/// Decodes RGB888 PICA texture data into a `Vec<u8>` of RGBA texture data.
///
/// # Arguments
///
/// * `texture_data` - A byte slice containing the raw texture data.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the decoded RGBA data.
///
fn decode_rgb888(texture_data: &[u8], width: u32, height: u32) -> Vec<u8> {
    println!("Decoding as RGB888");

    let bytes_per_pixel = 24 / 8;
    let mut output: Vec<u8> = vec![0; (width * height * 4) as usize];
    let mut src_idx: usize = 0;

    for ty in (0..height).step_by(8) {
        for tx in (0..width).step_by(8) {
            for px in SWIZZLE_LUT {

                let x = px & 7;
                let y = (px - x) >> 3;

                let out_idx = ((tx + x + (height - 1 - (ty + y)) * width) * 4) as usize;

                output[out_idx    ] = texture_data[src_idx + 2];
                output[out_idx + 1] = texture_data[src_idx + 1];
                output[out_idx + 2] = texture_data[src_idx    ];
                output[out_idx + 3] = 0xFF;

                src_idx += bytes_per_pixel;
            }
        }
    }
    output
}

/// Decodes RGBA5551 PICA texture data into a `Vec<u8>` of RGBA texture data.
///
/// # Arguments
///
/// * `texture_data` - A byte slice containing the raw texture data.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the decoded RGBA data.
///
fn decode_rgba5551(texture_data: &[u8], width: u32, height: u32) -> Vec<u8> {
    println!("Decoding as RGBA5551");

    let bytes_per_pixel = 16 / 8;
    let mut output: Vec<u8> = vec![0; (width * height * 4) as usize];
    let mut src_idx: usize = 0;

    for ty in (0..height).step_by(8) {
        for tx in (0..width).step_by(8) {
            for px in SWIZZLE_LUT {

                let x = px & 7;
                let y = (px - x) >> 3;

                let out_idx = ((tx + x + (height - 1 - (ty + y)) * width) * 4) as usize;
                let value = (texture_data[src_idx] as u16) | ((texture_data[src_idx + 1] as u16) << 8);

                let r = (((value >>  1) & 0x1F) << 3) as u8;
                let g = (((value >>  6) & 0x1F) << 3) as u8;
                let b = (((value >> 11) & 0x1F) << 3) as u8;
                let a = (value & 1) as u8;

                output[out_idx    ] = b | (b >> 5);
                output[out_idx + 1] = g | (g >> 5);
                output[out_idx + 2] = r | (r >> 5);
                output[out_idx + 3] = a * 0xFF;

                src_idx += bytes_per_pixel;
            }
        }
    }
    output
}

/// Decodes RGB565 PICA texture data into a `Vec<u8>` of RGBA texture data.
///
/// # Arguments
///
/// * `texture_data` - A byte slice containing the raw texture data.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the decoded RGBA data.
///
fn decode_rgb565(texture_data: &[u8], width: u32, height: u32) -> Vec<u8> {
    println!("Decoding as RGB565");

    let bytes_per_pixel = 16 / 8;
    let mut output: Vec<u8> = vec![0; (width * height * 4) as usize];
    let mut src_idx: usize = 0;

    for ty in (0..height).step_by(8) {
        for tx in (0..width).step_by(8) {
            for px in SWIZZLE_LUT {

                let x = px & 7;
                let y = (px - x) >> 3;

                let out_idx = ((tx + x + (height - 1 - (ty + y)) * width) * 4) as usize;
                let value = (texture_data[src_idx] as u16) | ((texture_data[src_idx + 1] as u16) << 8);

                let r = ((value & 0x1F) << 3) as u8;
                let g = (((value >>  5) & 0x3F) << 2) as u8;
                let b = (((value >> 11) & 0x1F) << 3) as u8;

                output[out_idx    ] = b | (b >> 5);
                output[out_idx + 1] = g | (g >> 6);
                output[out_idx + 2] = r | (r >> 5);
                output[out_idx + 3] = 0xFF;

                src_idx += bytes_per_pixel;
            }
        }
    }
    output
}

/// Decodes RGBA4444 PICA texture data into a `Vec<u8>` of RGBA texture data.
///
/// # Arguments
///
/// * `texture_data` - A byte slice containing the raw texture data.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the decoded RGBA data.
///
fn decode_rgba4444(texture_data: &[u8], width: u32, height: u32) -> Vec<u8> {
    println!("Decoding as RGBA4444");

    let bytes_per_pixel = 16 / 8;
    let mut output: Vec<u8> = vec![0; (width * height * 4) as usize];
    let mut src_idx: usize = 0;

    for ty in (0..height).step_by(8) {
        for tx in (0..width).step_by(8) {
            for px in SWIZZLE_LUT {

                let x = px & 7;
                let y = (px - x) >> 3;

                let out_idx = ((tx + x + (height - 1 - (ty + y)) * width) * 4) as usize;
                let value = (texture_data[src_idx] as u16) | ((texture_data[src_idx + 1] as u16) << 8);

                let r = ((value >>  4) & 0xF) as u8;
                let g = ((value >>  8) & 0xF) as u8;
                let b = ((value >> 12) & 0xF) as u8;
                let a = (value & 0xF) as u8;

                output[out_idx    ] = b | (b << 4);
                output[out_idx + 1] = g | (g << 4);
                output[out_idx + 2] = r | (r << 4);
                output[out_idx + 3] = a | (a << 4);

                src_idx += bytes_per_pixel;
            }
        }
    }
    output
}
