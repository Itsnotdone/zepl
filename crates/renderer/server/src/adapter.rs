use wgpu::{Adapter, Instance, Surface};

pub struct RenderingAdapter{
    pub adapter: Adapter
}

impl RenderingAdapter{
    pub fn new(instance: &Instance, surface: &Surface) -> Self{
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .expect("Failed to find an appropriate adapter");
        Self { adapter }
    }
}