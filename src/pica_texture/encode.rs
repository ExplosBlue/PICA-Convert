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
        TextureFormat::RGBA5551 => encode_rgba5551(&img, width, height),
        TextureFormat::RGB565   => encode_rgb565(&img, width, height),
        TextureFormat::RGBA4444 => encode_rgba4444(&img, width, height),
        TextureFormat::LA88     => encode_la88(&img, width, height),
        TextureFormat::HL8      => encode_hl8(&img, width, height),
        TextureFormat::L8       => encode_l8(&img, width, height),
        TextureFormat::A8       => encode_a8(&img, width, height),
        TextureFormat::LA44     => encode_la44(&img, width, height),
        TextureFormat::L4       => encode_l4(&img, width, height),
        TextureFormat::A4       => encode_a4(&img, width, height),
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

/// Encodes an RGBA image as RGBA5551 PICA texture data.
///
/// # Arguments
///
/// * `img` - A reference to the input image (`RgbaImage`) to encode.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded RGBA5551 data.
///
/// # Example
///
/// ```rust
/// # use image::RgbaImage;
/// # use pica_convert::pica_texture::encode::encode_rgba5551;
/// let img = RgbaImage::new(128, 128);
/// let encoded = encode_rgb5551(&img, 128, 128);
/// assert_eq!(encoded.len(), 128 * 128 * 2);
/// ```
fn encode_rgba5551(img: &RgbaImage, width: u32, height: u32) -> Vec<u8> {
    println!("Encoding as RGBA5551");

    let mut output: Vec<u8> = Vec::with_capacity(width as usize * height as usize * 2);

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

                let r = (pixel[0] >> 3) as u16;
                let g = (pixel[1] >> 3) as u16;
                let b = (pixel[2] >> 3) as u16;
                let a = if pixel[3] > 127 { 1 } else { 0 } as u16;
                let value = (r << 11) | (g << 6) | (b << 1) | a;

                output.extend([(value & 0xFF) as u8, (value >> 8) as u8]);
            }
        }
    }
    output
}

/// Encodes an RGBA image as RGB565 PICA texture data.
///
/// # Arguments
///
/// * `img` - A reference to the input image (`RgbaImage`) to encode.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded RGB565 data.
///
/// # Example
///
/// ```rust
/// # use image::RgbaImage;
/// # use pica_convert::pica_texture::encode::encode_rgb565;
/// let img = RgbaImage::new(128, 128);
/// let encoded = encode_rgb565(&img, 128, 128);
/// assert_eq!(encoded.len(), 128 * 128 * 2);
/// ```
pub fn encode_rgb565(img: &RgbaImage, width: u32, height: u32) -> Vec<u8> {
    println!("Encoding as RGB565");

    let mut output: Vec<u8> = Vec::with_capacity(width as usize * height as usize * 2);

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

                let r = (pixel[0] >> 3) as u16;
                let g = (pixel[1] >> 2) as u16;
                let b = (pixel[2] >> 3) as u16;
                let value = (r << 11) | (g << 5) | b;

                output.extend([(value & 0xFF) as u8, (value >> 8) as u8]);
            }
        }
    }
    output
}

/// Encodes an RGBA image as RGBA4444 PICA texture data.
///
/// # Arguments
///
/// * `img` - A reference to the input image (`RgbaImage`) to encode.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded RGBA4444 data.
///
/// # Example
///
/// ```rust
/// # use image::RgbaImage;
/// # use pica_convert::pica_texture::encode::encode_rgba4444;
/// let img = RgbaImage::new(128, 128);
/// let encoded = encode_rgba4444(&img, 128, 128);
/// assert_eq!(encoded.len(), 128 * 128 * 2);
/// ```
pub fn encode_rgba4444(img: &RgbaImage, width: u32, height: u32) -> Vec<u8> {
    println!("Encoding as RGBA4444");

    let mut output: Vec<u8> = Vec::with_capacity(width as usize * height as usize * 2);

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

                let r = (pixel[0] >> 4) as u16;
                let g = (pixel[1] >> 4) as u16;
                let b = (pixel[2] >> 4) as u16;
                let a = (pixel[3] >> 4) as u16;
                let value = (r << 12) | (g << 8) | (b << 4) | a;

                output.extend([(value & 0xFF) as u8, (value >> 8) as u8]);
            }
        }
    }
    output
}

/// Encodes an RGBA image as LA88 PICA texture data.
///
/// # Arguments
///
/// * `img` - A reference to the input image (`RgbaImage`) to encode.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded LA88 data.
///
/// # Example
///
/// ```rust
/// # use image::RgbaImage;
/// # use pica_convert::pica_texture::encode::encode_la88;
/// let img = RgbaImage::new(128, 128);
/// let encoded = encode_la88(&img, 128, 128);
/// assert_eq!(encoded.len(), 128 * 128 * 2);
/// ```
pub fn encode_la88(img: &RgbaImage, width: u32, height: u32) -> Vec<u8> {
    println!("Encoding as LA88");

    let mut output: Vec<u8> = Vec::with_capacity(width as usize * height as usize * 2);

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

                let r = pixel[0] as u32;
                let g = pixel[1] as u32;
                let b = pixel[2] as u32;
                let a = pixel[3];

                let l = ((r + g + b) / 3) as u8;

                output.extend([a, l]);
            }
        }
    }
    output
}

/// Encodes an RGBA image as HL8 PICA texture data.
///
/// # Arguments
///
/// * `img` - A reference to the input image (`RgbaImage`) to encode.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded HL8 data.
///
/// # Example
///
/// ```rust
/// # use image::RgbaImage;
/// # use pica_convert::pica_texture::encode::encode_hl8;
/// let img = RgbaImage::new(128, 128);
/// let encoded = encode_hl8(&img, 128, 128);
/// assert_eq!(encoded.len(), 128 * 128 * 2);
/// ```
pub fn encode_hl8(img: &RgbaImage, width: u32, height: u32) -> Vec<u8> {
    println!("Encoding as HL8");

    let mut output: Vec<u8> = Vec::with_capacity(width as usize * height as usize * 2);

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

                let l = pixel[0];
                let h = pixel[1];
                output.extend([h, l]);
            }
        }
    }
    output
}

/// Encodes an RGBA image as L8 PICA texture data.
///
/// # Arguments
///
/// * `img` - A reference to the input image (`RgbaImage`) to encode.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded L8 data.
///
/// # Example
///
/// ```rust
/// # use image::RgbaImage;
/// # use pica_convert::pica_texture::encode::encode_l8;
/// let img = RgbaImage::new(128, 128);
/// let encoded = encode_l8(&img, 128, 128);
/// assert_eq!(encoded.len(), 128 * 128);
/// ```
fn encode_l8(img: &RgbaImage, width: u32, height: u32) -> Vec<u8> {
    println!("Encoding as L8");

    let mut output: Vec<u8> = Vec::with_capacity(width as usize * height as usize);

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

                let r = pixel[0] as u32;
                let g = pixel[1] as u32;
                let b = pixel[2] as u32;

                let l = ((r + g + b) / 3) as u8;

                output.extend([l]);
            }
        }
    }
    output
}

/// Encodes an RGBA image as A8 PICA texture data.
///
/// # Arguments
///
/// * `img` - A reference to the input image (`RgbaImage`) to encode.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded A8 data.
///
/// # Example
///
/// ```rust
/// # use image::RgbaImage;
/// # use pica_convert::pica_texture::encode::encode_a8;
/// let img = RgbaImage::new(128, 128);
/// let encoded = encode_a8(&img, 128, 128);
/// assert_eq!(encoded.len(), 128 * 128);
/// ```
fn encode_a8(img: &RgbaImage, width: u32, height: u32) -> Vec<u8> {
    println!("Encoding as A8");

    let mut output: Vec<u8> = Vec::with_capacity(width as usize * height as usize);

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

                let a = pixel[3];
                output.extend([a]);
            }
        }
    }
    output
}

/// Encodes an RGBA image as LA44 PICA texture data.
///
/// # Arguments
///
/// * `img` - A reference to the input image (`RgbaImage`) to encode.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded LA44 data.
///
/// # Example
///
/// ```rust
/// # use image::RgbaImage;
/// # use pica_convert::pica_texture::encode::encode_la44;
/// let img = RgbaImage::new(128, 128);
/// let encoded = encode_la44(&img, 128, 128);
/// assert_eq!(encoded.len(), 128 * 128);
/// ```
fn encode_la44(img: &RgbaImage, width: u32, height: u32) -> Vec<u8> {
    println!("Encoding as LA44");

    let mut output: Vec<u8> = Vec::with_capacity(width as usize * height as usize);

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

                let r = pixel[0] as u32;
                let g = pixel[1] as u32;
                let b = pixel[2] as u32;

                let l = (((r + g + b) / 3) >> 4) as u8;
                let a = pixel.0[3] >> 4;

                output.extend([(l << 4) | a]);
            }
        }
    }
    output
}

/// Encodes an RGBA image as L4 PICA texture data.
///
/// # Arguments
///
/// * `img` - A reference to the input image (`RgbaImage`) to encode.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded L4 data.
///
/// # Example
///
/// ```rust
/// # use image::RgbaImage;
/// # use pica_convert::pica_texture::encode::encode_l4;
/// let img = RgbaImage::new(128, 128);
/// let encoded = encode_l4(&img, 128, 128);
/// assert_eq!(encoded.len(), 128 * 128);
/// ```
fn encode_l4(img: &RgbaImage, width: u32, height: u32) -> Vec<u8> {
    println!("Encoding as L4");

    let mut output: Vec<u8> = vec![0; width as usize * height as usize];

    let mut dst_index = 0;

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

                let pixel = img.get_pixel(img_x, img_y).to_rgba();

                let r = pixel[0] as u32;
                let g = pixel[1] as u32;
                let b = pixel[2] as u32;

                let l = (((r + g + b) / 3) >> 4) as u8;

                let byte_index = dst_index >> 1;
                let shift = (dst_index & 1) << 2;

                output[byte_index] &= !(0xF << shift);
                output[byte_index] |= (l & 0xF) << shift;

                dst_index += 1;
            }
        }
    }
    output
}

/// Encodes an RGBA image as A4 PICA texture data.
///
/// # Arguments
///
/// * `img` - A reference to the input image (`RgbaImage`) to encode.
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded A4 data.
///
/// # Example
///
/// ```rust
/// # use image::RgbaImage;
/// # use pica_convert::pica_texture::encode::encode_a4;
/// let img = RgbaImage::new(128, 128);
/// let encoded = encode_a4(&img, 128, 128);
/// assert_eq!(encoded.len(), 128 * 128);
/// ```
fn encode_a4(img: &RgbaImage, width: u32, height: u32) -> Vec<u8> {
    println!("Encoding as A4");

    let mut output: Vec<u8> = vec![0; width as usize * height as usize];

    let mut dst_index = 0;

    for ty in (0..height).step_by(8) {
        for tx in (0..width).step_by(8) {
            for px in SWIZZLE_LUT {

                let x = px & 7;
                let y = (px >> 3) & 7;

                let img_x = tx + x;
                let img_y = ty + y;

                if img_x >= width || img_y >= height {
                    continue;
                }

                let pixel = img.get_pixel(img_x, img_y);

                let a = pixel[3] >> 4;

                let byte_index = dst_index >> 1;
                let shift = (dst_index & 1) << 2;

                output[byte_index] &= !(0xF << shift);
                output[byte_index] |= (a & 0xF) << shift;

                dst_index += 1;
            }
        }
    }
    output
}
