use serde::{Deserialize, Serialize};
use clap::ValueEnum;

#[derive(Clone, Debug)]
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