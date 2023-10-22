use wgpu::{Device, Adapter, Queue};

pub struct RenderingDevice {
    pub device: Device,
}

impl RenderingDevice {
    pub fn create_device_and_queue(adapter: &Adapter) -> (Self, RenderingQueue){
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits:
                    wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            },
            None,
        ))
        .expect("Failed to create device");

        (Self{device}, RenderingQueue{queue})
    }
}

pub struct RenderingQueue {
    pub queue: Queue,
}

impl RenderingQueue {
    pub fn new(queue: Queue) -> Self {
        Self { queue }
    }
}

