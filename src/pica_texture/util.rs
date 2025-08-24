
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
