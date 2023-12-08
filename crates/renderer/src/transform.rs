use scene::Transform2d;
use server::RenderingServer;
use wgpu::{*, util::{DeviceExt, BufferInitDescriptor}};

pub(crate) trait TransformMethods{
    fn prepare(&mut self, server: &mut RenderingServer) -> BindGroup;
}

impl TransformMethods for Transform2d{
    fn prepare(&mut self, server: &mut RenderingServer) -> BindGroup {
        let mut buffer = encase::DynamicStorageBuffer::new(Vec::new());

        buffer.write(&self.into_raw(server.window_size)).unwrap();

        let transform_buffer = server.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(&buffer.into_inner()),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bindgroup = server.device.create_bind_group(&BindGroupDescriptor {
            layout: &server.transform_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
            label: Some("bind_group"),
        });

        bindgroup
    }
}