use glfw::Window;
use wgpu::{Surface, Instance};

pub struct RenderingSurface{
    pub surface: Surface
}

impl RenderingSurface{
    pub fn new(instance: &Instance, window: &Window) -> Self{
        let surface = unsafe { instance.create_surface(window).unwrap() };

        Self { surface }
    }
}