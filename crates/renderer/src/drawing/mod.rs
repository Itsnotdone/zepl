mod entity;

pub use entity::*;

use server::{RenderingServer, RenderObject};
use ecs::Resources;

pub trait Draw {
    fn draw(
        &mut self,
        server: &mut RenderingServer,
        resources: &mut Resources,
    ) -> Option<RenderObject>;
}