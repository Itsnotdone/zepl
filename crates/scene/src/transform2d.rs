pub use glam::*;

use serde::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform2d {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Transform2d {
    pub fn new() -> Self {
        Self {
            position: vec2(0.0, 0.0),
            rotation: 0.0,
            scale: vec2(0.0, 0.0),
        }
    }
}
