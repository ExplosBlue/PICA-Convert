use serde::{Deserialize, Serialize};
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum TextureFormat {
    RGBA8888,
    RGB888,
    RGBA5551,
    RGB565,
    RGBA4444,
    LA88,
    HL8,
    L8,
    A8,
    LA44,
    L4,
    A4,
    ETC1,
    ETC1A4
}

impl<'de> Deserialize<'de> for TextureFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let normalized = s.replace('_', "").to_uppercase();

        match normalized.as_str() {
            "RGBA8888" => Ok(TextureFormat::RGBA8888),
            "RGB888" => Ok(TextureFormat::RGB888),
            "RGBA5551" => Ok(TextureFormat::RGBA5551),
            "RGB565" => Ok(TextureFormat::RGB565),
            "RGBA4444" => Ok(TextureFormat::RGBA4444),
            "LA88" => Ok(TextureFormat::LA88),
            "HL8" => Ok(TextureFormat::HL8),
            "L8" => Ok(TextureFormat::L8),
            "A8" => Ok(TextureFormat::A8),
            "LA44" => Ok(TextureFormat::LA44),
            "L4" => Ok(TextureFormat::L4),
            "A4" => Ok(TextureFormat::A4),
            "ETC1" => Ok(TextureFormat::ETC1),
            "ETC1A4" => Ok(TextureFormat::ETC1A4),
            other => Err(serde::de::Error::custom(format!("Unknown TextureFormat: {}", other))),
        }
    }
}

impl Serialize for TextureFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer,
    {
        let s = match self {
            TextureFormat::RGBA8888 => "Rgba8888",
            TextureFormat::RGB888   => "Rgb888",
            TextureFormat::RGBA5551 => "Rgba5551",
            TextureFormat::RGB565   => "Rgb565",
            TextureFormat::RGBA4444 => "Rgba4444",
            TextureFormat::LA88     => "La88",
            TextureFormat::HL8      => "Hl8",
            TextureFormat::L8       => "L8",
            TextureFormat::A8       => "A8",
            TextureFormat::LA44     => "La44",
            TextureFormat::L4       => "L4",
            TextureFormat::A4       => "A4",
            TextureFormat::ETC1     => "Etc1",
            TextureFormat::ETC1A4   => "Etc1_a4",
        };
        serializer.serialize_str(s)
    }
}

pub struct PicaTexture {
    format: TextureFormat,
    width: u32,
    height: u32,
    data: Vec<u8>
}

impl PicaTexture {
    pub fn new(format: TextureFormat, width: u32, height: u32, data: Vec<u8>) -> Self {
        Self {
            format,
            width,
            height,
            data
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn format(&self) -> &TextureFormat {
        &self.format
    }

}