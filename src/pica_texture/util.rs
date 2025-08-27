
/// Vertically Flips a buffer of raw texture data.
///
/// # Arguments
/// 
/// * `data` - A byte slice containing the raw texture data.
/// * `width` - The width of the texture in pixels.
/// * `height` - The height of the texture in pixels.
/// 
pub fn flip_vertical(data: &mut [u8], width: u32, height: u32) {
    let row_bytes =(width * 4) as usize;
    let img_size = row_bytes * height as usize;
    assert_eq!(data.len(), img_size);

    for y in 0..(height as usize / 2) {
        let top_start = y * row_bytes;
        let bot_start = (height as usize - 1 - y) * row_bytes;

        let (top, bot) = data.split_at_mut(bot_start);
        top[top_start..top_start + row_bytes].swap_with_slice(&mut bot[..row_bytes]);
    }
}

pub(crate) const XT: [u32; 4] = [0, 4, 0, 4];
pub(crate) const YT: [u32; 4] = [0, 0, 4, 4];

pub(crate) const SWIZZLE_LUT: [u32; 64] = [
    0,  1,  8,  9,  2,  3,  10, 11,
    16, 17, 24, 25, 18, 19, 26, 27,
    4,  5,  12, 13,  6,  7, 14, 15,
    20, 21, 28, 29, 22, 23, 30, 31,
    32, 33, 40, 41, 34, 35, 42, 43,
    48, 49, 56, 57, 50, 51, 58, 59,
    36, 37, 44, 45, 38, 39, 46, 47,
    52, 53, 60, 61, 54, 55, 62, 63
];

/// Swaps the byte order of a byte array.
///
/// # Arguments
/// 
/// * `bytes` - The byte array to swap.
/// 
/// # Returns
/// 
/// A `[u8; 8]` containing the swapped bytes.
/// 
pub(crate) fn swap64(bytes: [u8; 8]) -> [u8; 8] {
    let value = u64::from_le_bytes(bytes);
    let swapped = value.swap_bytes();
    swapped.to_le_bytes()
}
