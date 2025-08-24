use image::{DynamicImage, GenericImageView, RgbaImage};

use crate::pica_texture::types::{TextureFormat, SWIZZLE_LUT};

/// Encodes a [`DynamicImage`] into raw PICA texture data for a given [`TextureFormat`].
///
/// This function converts the input image to `RGBA8` internally to ensure
/// consistent pixel access, then dispatches to the appropriate encoder
/// depending on the requested texture format.
///
/// # Arguments
///
/// * `img` - The source image to encode.
/// * `format` - The target [`TextureFormat`] specifying how the image should be encoded.
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded texture data on success,
/// or an error if encoding fails.
///
/// # Errors
///
/// Returns an error if an unsupported or unimplemented texture format is requested.
///
/// # Example
///
/// ```
/// # use image::DynamicImage;
/// # use pica_convert::pica_texture::{encode::encode_texture, TextureFormat};
/// // Create a blank 4x4 RGBA image
/// let img = DynamicImage::new_rgba8(32, 32);
///
/// // Encode the image into RGBA8888 format
/// let encoded = encode_texture(&img, &TextureFormat::RGBA8888).unwrap();
///
/// // Each pixel is 4 bytes in RGBA8888
/// assert_eq!(encoded.len(), 32 * 32 * 4);
/// ```
pub fn encode_texture(img: &DynamicImage, format: &TextureFormat) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let (width, height) = img.dimensions();
    println!("Encoding texture with dimensions {}x{}", width, height);

    // Ensure image is rgba8 before doing any encoding
    let img = img.to_rgba8();

    let output_texture = match format {
        TextureFormat::RGBA8888 => encode_rgba8888(&img, width, height),
        TextureFormat::RGB888   => encode_rgb888(&img, width, height),
        _ => unimplemented!("Encoding for the specified format is not implemented yet"),
    };
    Ok(output_texture)
}

/// Encodes an RGBA image as RGBA8888 PICA texture data.
///
/// # Arguments
///
/// * `img` - A reference to the input image (`RgbaImage`) to encode.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded RGBA8888 data.
///
/// # Example
///
/// ```rust
/// # use image::RgbaImage;
/// # use pica_convert::pica_texture::encode::encode_rgba8888;
/// let img = RgbaImage::new(128, 128);
/// let encoded = encode_rgba8888(&img, 128, 128);
/// assert_eq!(encoded.len(), 128 * 128 * 4);
/// ```
fn encode_rgba8888(img: &RgbaImage, width: u32, height: u32) -> Vec<u8> {
    println!("Encoding as RGBA8888");

    let mut output: Vec<u8> = Vec::with_capacity(width as usize * height as usize * 4);

    for ty in (0..height).step_by(8) {
        for tx in (0..width).step_by(8) {
            for &px in SWIZZLE_LUT.iter() {

                let x = px & 7;
                let y = (px >> 3) & 7;

                let img_x = tx + x;
                let img_y = ty + y;

                if img_x >= width || img_y >= height {
                    continue;
                }

                let pixel = img.get_pixel(img_x, img_y);
                output.extend([pixel[3], pixel[2], pixel[1], pixel[0]]);
            }
        }
    }
    output
}

/// Encodes an RGBA image as RGB888 PICA texture data.
///
/// # Arguments
///
/// * `img` - A reference to the input image (`RgbaImage`) to encode.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded RGB888 data.
///
/// # Example
///
/// ```rust
/// # use image::RgbaImage;
/// # use pica_convert::pica_texture::encode::encode_rgb888;
/// let img = RgbaImage::new(128, 128);
/// let encoded = encode_rgb888(&img, 128, 128);
/// assert_eq!(encoded.len(), 128 * 128 * 3);
/// ```
fn encode_rgb888(img: &RgbaImage, width: u32, height: u32) -> Vec<u8> {
    println!("Encoding as RGB888");

    let mut output: Vec<u8> = Vec::with_capacity(width as usize * height as usize * 3);

    for ty in (0..height).step_by(8) {
        for tx in (0..width).step_by(8) {
            for &px in SWIZZLE_LUT.iter() {

                let x = px & 7;
                let y = (px >> 3) & 7;

                let img_x = tx + x;
                let img_y = ty + y;

                if img_x >= width || img_y >= height {
                    continue;
                }

                let pixel = img.get_pixel(img_x, img_y);
                output.extend([pixel[2], pixel[1], pixel[0]]);
            }
        }
    }
    output
}
