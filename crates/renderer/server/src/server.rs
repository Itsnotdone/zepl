use glfw::Window;

use crate::{RenderingInstance, RenderingAdapter, RenderingDevice, RenderingSurface};

pub struct RenderingServer {}

impl RenderingServer {
    pub fn new(window: &Window) -> Self {
        let instance = RenderingInstance::new();
        let surface = RenderingSurface::new(&instance.instance, window);

        let adapter = RenderingAdapter::new(&instance.instance, &surface.surface);

        let (_device, _queue) = RenderingDevice::create_device_and_queue(&adapter.adapter);
    
        Self {}
    }
}
