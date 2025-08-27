use std::mem::MaybeUninit;
use std::sync::Once;

static INIT: Once = Once::new();

#[repr(C)]
pub struct Etc1PackParams {
    pub quality: i32,
    pub dithering: i32,
}

#[link(name = "rg_etc1_wrapper")]
unsafe extern "C" {
    fn etc1_pack_init();
    fn etc1_compress_block(rgba_pixels: *const u32, out_block: *mut u8, params: *const Etc1PackParams) -> u32;
    fn etc1_decompress_block(etc1_block: *const u8, out_rgba: *mut u32, preserve_alpha: i32) -> i32;
}

/// Initializes the ETC1 compressor.
/// 
/// Must be called before compressing any blocks.
/// 
fn init() {
    unsafe { etc1_pack_init() }
}

/// Ensures that the ETC1 compressor has been initialized.
/// 
fn ensure_initialized() {
    INIT.call_once(|| {
        init();
    });
}

/// Compresses a single 4x4 block of RGBA pixels into an 8-byte ETC1 block.
///
/// # Arguments
///
/// * `rgba` - A block of 16 8-bit RGBA pixels to be compressed.
/// * `params` - Parameters to be used when compressing the block.
/// 
/// # Returns
/// A `[u8; 8]` containing the compressed block.
/// 
pub fn compress_block(rgba: &[u8; 64], params: Option<Etc1PackParams>) -> [u8; 8] {
    ensure_initialized();

    let mut out_block = [0u8; 8];

    let rgba32: &[u32; 16] = unsafe { &*(rgba.as_ptr() as *const [u32; 16]) };

    let param_ptr = match params {
        Some(ref p) => p as *const Etc1PackParams,
        None => std::ptr::null(),
    };

    unsafe {
        etc1_compress_block(rgba32.as_ptr(), out_block.as_mut_ptr(), param_ptr);
    }

    out_block
}

/// Decompresses a single 8-byte block of ETC1 data into a 4x4 block of RGBA pixels.
///
/// # Arguments
///
/// * `block` - The block of ETC1 data to decompress.
/// * `preserve_alpha` - Determines whether to keep existing alpha values. If false, alpha is set to 255.
/// 
/// # Returns
/// A `[u8; 64]` containing the decompressed RGBA data.
/// 
pub fn decompress_block(block: &[u8; 8], preserve_alpha: bool) -> [u8; 64] {
    ensure_initialized();

    let mut out: [MaybeUninit<u32>; 16] = unsafe { MaybeUninit::uninit().assume_init() };

    unsafe {
        etc1_decompress_block(
            block.as_ptr(),
            out.as_mut_ptr() as *mut u32,
            preserve_alpha as i32,
        );
    }

    let out_bytes: [u8; 16 * 4] = unsafe { std::mem::transmute(out) };
    out_bytes
}

pub mod quality {
    pub const LOW: i32 = 0;
    pub const MEDIUM: i32 = 1;
    pub const HIGH: i32 = 2;
}