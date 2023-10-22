use wgpu::Instance;

pub struct RenderingInstance {
    pub instance: Instance,
}

impl RenderingInstance {
    pub fn new() -> Self {
        let instance = Instance::default();

        Self { instance }
    }
}
