# PICA Convert

**PICA Convert** is a command-line utility and library for converting textures into
**Nintendo 3DS texture formats**.

## Features

- Encode standard images into 3DS-compatible texture formats  
- Decode raw 3DS texture data back into common image formats  

## Installation

### From Source
```sh
git clone https://github.com/ExplosBlue/PICA-Convert.git
cd pica-convert
git submodule update --init
cargo build --release
```

The binary will be available in target/release/pica-convert.

## Usage

### Command Line
```sh
pica-convert -m encode -i input.png -o output.ctex -f rgba8888
pica-convert -m decode -i input.ctex -o output.png
```

### As a Library
```sh
use pica_convert::pica_texture::{encode_texture, decode_texture, TextureFormat};
use image::DynamicImage;

// Example: Encode an image
let img = image::open("input.png")?;
let encoded = encode_texture(&img, &TextureFormat::RGBA8888)?;

// Example: Decode raw data
let decoded = decode_texture(&encoded)?;
decoded.save("output.png")?;
```

## Benchmarks

Benchmarks use [criterion](https://crates.io/crates/criterion) and run against release builds.

```sh
# Run all benchmarks
cargo bench

# Run a specific benchmark target
cargo bench --bench encode
cargo bench --bench decode

# Run a single benchmark
cargo bench --bench encode -- "encode_etc1 512x512"
```

Criterion caches results between runs and reports performance changes automatically.

## Testing

Tests cover every encoder/decoder and use known-good samples under `tests/known_good/`..

```sh
# Run all tests
cargo test

# Regenerate known-good fixtures after a deliberate format change
PICA_REGEN_KNOWN_GOOD=1 cargo test
```

## License
This project is licensed under the MIT License.
See LICENSE for details.

## Acknowledgements
- [image](https://crates.io/crates/image) - used for image handling.
- [clap](https://crates.io/crates/clap) - used for arg parsing.
- [serde_xml_rs](https://crates.io/crates/serde_xml_rs) - used for XML serialization.
- [base64](https://crates.io/crates/base64) - used for base64 encoding/decoding.
- [ExETC1](https://github.com/ExplosBlue/ExETC1) - used for etc1 encoding/decoding.
