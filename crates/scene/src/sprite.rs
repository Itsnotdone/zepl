use ecs::Handle;
use serde::{Deserialize, Serialize};

use crate::Texture;

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sprite {
    pub texture: Handle<Texture>,
}

impl Sprite {
    pub fn new(texture: Handle<Texture>) -> Self {
        Self {
            texture,
        }
    }
}
