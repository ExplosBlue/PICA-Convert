use clap::Parser;
use image::{DynamicImage, GenericImageView, ImageReader};

use pica_convert::{pica_texture, serialization};

#[derive(Parser)]
struct Cli {
    #[arg(short = 'i', long, help = "Input image file")]
    input_file: String,

    #[arg(short = 'm', long, value_enum, help = "Operation mode: encode or decode")]
    mode: Mode,

    #[arg(short = 'f', long, help = "Output texture format")]
    output_format: Option<pica_texture::TextureFormat>,

    #[arg(short = 'r', long, help = "Resize image to nearest power of two if not already")]
    resize: bool,

    #[arg(short = 'o', long, help = "Output file name")]
    output_file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let status = match args.mode {
        Mode::Encode => encode_texture(args),
        Mode::Decode => decode_texture(args),
    };

    if let Err(e) = status {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

fn encode_texture(args: Cli) -> Result<(), Box<dyn std::error::Error>> {
    // Load image
    let mut img: DynamicImage = match ImageReader::open(&args.input_file) {
        Ok(reader) => match reader.decode() {
            Ok(image) => image,
            Err(e) => {
                return Err(format!("Failed to decode image: {}", e).into());
            }
        },
        Err(e) => {
            return Err(format!("Failed to open image file '{}': {}", args.input_file, e).into());
        }
    };

    // Check input texture is power of two
    let (width, height) = img.dimensions();
    if (width & (width - 1)) != 0 || (height & (height - 1)) != 0 {
        if args.resize {
            let new_width = width.next_power_of_two();
            let new_height = height.next_power_of_two();
            println!("Resizing image from {}x{} to {}x{}", width, height, new_width, new_height);
            img = img.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3);
        } else {
            return Err("Image dimensions are not power of two".into());
        }
    }

    if width > 1024 || height > 1024 {
        return Err("Image dimensions must not exceed 1024x1024".into());
    }

    // Encode texture
    let output_format = match args.output_format.as_ref() {
        Some(fmt) => fmt,
        None => {
            return Err("Output format is required for encoding.".into());
        }
    };
    let encoded_texture = match pica_texture::encode_texture(&img, output_format) {
        Ok(tex) => tex,
        Err(e) => {
            return Err(format!("Failed to encode texture: {}", e).into());
        }
    };

    // Write file
    // TODO: For now this assumes we are writing a CTEX file
    serialization::ctex::serialize(encoded_texture, args.output_file);
    print!("File written successfully");
    Ok(())
}

fn decode_texture(args: Cli) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: For now this assumes we are decoding a CTEX file
    let encoded_texture = serialization::ctex::deserialize(args.input_file)?;

    let dec_texture = pica_texture::decode_texture(&encoded_texture)?;
    dec_texture.save(args.output_file)?;

    Ok(())
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Mode {
    Encode,
    Decode
}