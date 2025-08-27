use serde::{Deserialize, Serialize};
use serde_xml_rs::from_reader;
use std::fs::File;
use base64::{Engine as _, engine::{general_purpose}};

use crate::pica_texture::{PicaTexture, TextureFormat};

#[derive(Debug, Deserialize, Serialize)]
struct NintendoWareIntermediateFile {
    #[serde(rename = "GraphicsContentCtr")]
    graphics_content: GraphicsContentCtr,
}

#[derive(Debug, Deserialize, Serialize)]
struct GraphicsContentCtr {
    #[serde(rename = "Version", default)]
    version: String,
    #[serde(rename = "Namespace", default)]
    namespace: String,

    // This block is just used for metadata and seemingly isn't required for anything
    // #[serde(rename = "EditData")]
    // edit_data: Option<EditData>,

    #[serde(rename = "Textures")]
    textures: Textures
}

#[derive(Debug, Deserialize, Serialize)]
struct Textures {
    #[serde(rename = "ImageTextureCtr")]
    image_texture: ImageTextureCtr,
}

#[derive(Debug, Deserialize, Serialize)]
struct ImageTextureCtr {
    #[serde(rename = "@Name", default)]
    name: String,
    #[serde(rename = "@Width", default)]
    width: u32,
    #[serde(rename = "@Height", default)]
    height: u32,
    #[serde(rename = "@MipmapSize", default)]
    mipmap_size: u32,
    #[serde(rename = "@Path", default)]
    path: String,
    #[serde(rename = "@Encoding", default)]
    encoding: String,
    #[serde(rename = "@Format")]
    format: TextureFormat,
    #[serde(rename = "Images")]
    images: Images,
}

#[derive(Debug, Deserialize, Serialize)]
struct Images {
    #[serde(rename = "PixelBasedImageCtr")]
    pixel_data: String,
}

pub fn deserialize(path: String) -> Result<PicaTexture, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let ctex: NintendoWareIntermediateFile = from_reader(file)?;

    let texture = ctex.graphics_content.textures.image_texture;

    // TODO: Maybe support other encoding types assuming ctex supports encodings other than base64

    let data = general_purpose::STANDARD.decode(texture.images.pixel_data)?;
    let result = PicaTexture::new(texture.format, texture.width, texture.height, data);
    Ok(result)
}

pub fn serialize(texture: PicaTexture, filepath: String) {
    let ctex = NintendoWareIntermediateFile {
        graphics_content: GraphicsContentCtr {
            version: "1.3.0".to_string(),
            namespace: "".to_string(),
            textures: Textures {
                image_texture: ImageTextureCtr {
                    name: "".to_string(),
                    width: texture.width(),
                    height: texture.height(),
                    mipmap_size: 1, // TODO: Don't hardcode
                    path: filepath.clone(),
                    encoding: "Base64".to_string(),
                    format: texture.format().clone(),
                    images: Images {
                        pixel_data: general_purpose::STANDARD.encode(texture.data()),
                    },
                },
            },
        },
    };

    let mut file = File::create(filepath).expect("Failed to create file");

    // TODO: Proper error handling
    let _ = serde_xml_rs::to_writer(&mut file, &ctex);
}
