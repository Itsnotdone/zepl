use image::{DynamicImage, GenericImageView, RgbaImage};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Texture {
    pub size: (u32, u32),
    pub data: RgbaImage,
    desc: TextureDescription,
    bid: Option<u32>
}

impl Texture {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let image = image::load_from_memory(bytes).unwrap();
        Self {
            size: image.dimensions(),
            data: image.to_rgba8(),
            desc: TextureDescription::default(),
            bid: None
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct TextureDescription{
    filter: TextureFilter
}
#[derive(Debug, Default, Clone, PartialEq)]
pub enum TextureFilter{
    #[default]
    Linear,
    Nearest
}

pub mod texture_utils{
    use crate::Texture;

    impl Texture{
        pub fn get_bid(&self) -> Option<u32>{
            self.bid
        }

        pub fn set_bid(&mut self, bid: u32){
            self.bid = Some(bid);
        }
    }
}
