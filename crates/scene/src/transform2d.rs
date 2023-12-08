pub use glam::*;

use bytemuck::{Pod, Zeroable};
use encase::ShaderType;

use serde::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform2d {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
    bid: Option<u32>
}

impl Transform2d {
    pub fn new() -> Self {
        Self {
            position: vec2(0.0, 0.0),
            rotation: 0.0,
            scale: vec2(1.0, 1.0),
            bid: None
        }
    }

    pub fn to_matrix(&self, window_size: (i32, i32)) -> Mat4 {
        let fixed_position = vec2(
            self.position.x / window_size.0 as f32,
            self.position.y / window_size.1 as f32,
        );

        Mat4::from_scale_rotation_translation(
            self.scale.extend(1.0),
            Quat::from_rotation_z(self.rotation.to_radians()),
            fixed_position.extend(0.0),
        )
    }

    pub fn into_raw(&self, window_size: (i32, i32)) -> RawTransform2d {
        RawTransform2d {
            matrix: self.to_matrix(window_size),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, ShaderType)]
pub struct RawTransform2d {
    pub matrix: Mat4,
}


pub mod transform_utils{
    use crate::Transform2d;

    impl Transform2d{
        pub fn get_bid(&self) -> Option<u32>{
            self.bid
        }

        pub fn set_bid(&mut self, bid: u32){
            self.bid = Some(bid);
        }
    }
}
