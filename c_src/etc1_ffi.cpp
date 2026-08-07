#include <stdint.h>
#include "Etc1.h"

extern "C" {

// C-compatible enum for quality
enum etc1_quality_c {
    ETC1_QUALITY_LOW = 0,
    ETC1_QUALITY_MEDIUM = 1,
    ETC1_QUALITY_HIGH = 2
};

// C-compatible struct for pack params
typedef struct {
    int quality;       // corresponds to etc1_quality_c
    int dithering;     // 0 = false, 1 = true
} etc1_pack_params_c;

// Compress a single 4x4 block
unsigned int etc1_compress_block(const uint32_t* rgba_pixels, uint8_t* out_block, etc1_pack_params_c* params) {
    Etc1::Etc1PackParams cpp_params;
    if (params) {
        cpp_params.mDithering = params->dithering != 0;
        switch (params->quality) {
            case ETC1_QUALITY_LOW: cpp_params.mQuality = Etc1::Etc1Quality::Low; break;
            case ETC1_QUALITY_MEDIUM: cpp_params.mQuality = Etc1::Etc1Quality::Medium; break;
            case ETC1_QUALITY_HIGH: cpp_params.mQuality = Etc1::Etc1Quality::High; break;
            default: cpp_params.mQuality = Etc1::Etc1Quality::High; break;
        }
    }
    return Etc1::packEtc1Block(out_block, rgba_pixels, cpp_params);
}

// Decompress a single 8-byte ETC1 block
// Returns 1 if valid, 0 if invalid
int etc1_decompress_block(const uint8_t* etc1_block, uint32_t* out_rgba, int preserve_alpha) {
    bool valid = Etc1::unpackEtc1Block(etc1_block, out_rgba, preserve_alpha != 0);
    return valid ? 1 : 0;
}

} // extern "C"
