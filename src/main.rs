use std::{fs, path::{Path, PathBuf}};

use clap::Parser;
use image::{DynamicImage, GenericImageView, ImageReader};

use pica_convert::{pica_texture, serialization};

use rayon::prelude::*;

#[derive(Parser, Clone)]
struct Cli {
    #[arg(short = 'i', long, help = "Input image file or directory")]
    input_path: String,

    #[arg(short = 'm', long, value_enum, help = "Operation mode: encode or decode")]
    mode: Mode,

    #[arg(short = 'f', long, help = "Output texture format")]
    output_format: Option<pica_texture::TextureFormat>,

    #[arg(short = 'r', long, help = "Resize image to nearest power of two if not already")]
    resize: bool,

    #[arg(short = 'o', long, help = "Output file or directory")]
    output_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let input_metadata = fs::metadata(&args.input_path)?;
    
    if input_metadata.is_dir() {
        // Bulk processing

        let output_dir = Path::new(&args.output_path);
        fs::create_dir_all(output_dir)?;

        let files: Vec<PathBuf> = fs::read_dir(&args.input_path)?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|p| p.is_file())
            .collect();

        files.par_iter().for_each(|path| {
            let file_stem = path.file_stem().unwrap().to_string_lossy();

            let output_file = match args.mode {
                // TODO: Allow file type to be specified somehow
                Mode::Encode => output_dir.join(format!("{}.ctex", file_stem)),
                Mode::Decode => output_dir.join(format!("{}.png", file_stem)),
            };

            let file_args = Cli {
                input_path: path.to_string_lossy().to_string(),
                output_path: output_file.to_string_lossy().to_string(),
                ..args.clone()
            };

            if let Err(e) = match file_args.mode {
                Mode::Encode => encode_texture(file_args.clone()),
                Mode::Decode => decode_texture(file_args.clone()),
            } {
                eprintln!("Failed to process '{}': {}", path.display(), e);
            }
        });
    } else {
        // Single file
        match args.mode {
            Mode::Encode => encode_texture(args)?,
            Mode::Decode => decode_texture(args)?,
        };
    }

    println!("Conversion complete");

    Ok(())
}

fn encode_texture(args: Cli) -> Result<(), Box<dyn std::error::Error>> {
    // Load image
    let mut img: DynamicImage = match ImageReader::open(&args.input_path) {
        Ok(reader) => match reader.decode() {
            Ok(image) => image,
            Err(e) => {
                return Err(format!("Failed to decode image: {}", e).into());
            }
        },
        Err(e) => {
            return Err(format!("Failed to open image file '{}': {}", args.input_path, e).into());
        }
    };

    // Check input texture is power of two
    let (width, height) = img.dimensions();
    if (width & (width - 1)) != 0 || (height & (height - 1)) != 0 {
        if args.resize {
            let new_width = width.next_power_of_two();
            let new_height = height.next_power_of_two();
            println!("Resizing image '{}' from {}x{} to {}x{}", args.input_path, width, height, new_width, new_height);
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
    serialization::ctex::serialize(encoded_texture, args.output_path.clone());
    println!("Encoded file written to '{}'", args.output_path);
    Ok(())
}

fn decode_texture(args: Cli) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: For now this assumes we are decoding a CTEX file
    let encoded_texture = serialization::ctex::deserialize(args.input_path)?;

    let dec_texture = pica_texture::decode_texture(&encoded_texture)?;

    dec_texture.save(args.output_path.clone())?;
    println!("Decoded file written to '{}'", args.output_path);

    Ok(())
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Mode {
    Encode,
    Decode
}