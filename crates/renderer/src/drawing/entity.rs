use wgpu::*;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use ecs::*;
use scene::*;

use server::{RenderObject, RenderingServer, Vertex, RawVertex};

use crate::{Draw, TextureMethods, TransformMethods};

impl Draw for Entity {
    fn draw(
        &mut self,
        server: &mut RenderingServer,
        resources: &mut Resources,
    ) -> Option<RenderObject>{
        if !self.has::<Transform2d>() || !self.has::<Sprite>(){
            return None;
        }
        let transform  = self.get_mut::<Transform2d>().unwrap();
        let transform_bg = transform.prepare(server);

        let sprite = self.get::<Sprite>().unwrap();
        let texture = resources.handle_mut(&sprite.texture).unwrap();
        
        let texture_bid = texture.prepare(server);

        let width = texture.size.0 as f32 / server.window_size.0 as f32;
        let height = texture.size.1 as f32 / server.window_size.1 as f32;

        let vertices: Vec<RawVertex> = vec![
            Vertex::new(-width, -height, 0.0)
                .with_tex_coords(1.0, 1.0)
                .into_raw(),
            Vertex::new(width, -height, 0.0)
                .with_tex_coords(0.0, 1.0)
                .into_raw(),
            Vertex::new(width, height, 0.0)
                .with_tex_coords(0.0, 0.0)
                .into_raw(),
            Vertex::new(-width, height, 0.0)
                .with_tex_coords(1.0, 0.0)
                .into_raw(),
        ];

        let indices: Vec<u16> = vec![0, 1, 2, 2, 3, 0];

        let v_len = vertices.len() as u32;
        let i_len = indices.len() as u32;

        let v_buffer = server.device.create_buffer_init(&BufferInitDescriptor{
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX
        });
        let i_buffer = server.device.create_buffer_init(&BufferInitDescriptor{
            label: None,
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX
        });

        return Some(RenderObject { v_len, v_buffer, i_len, i_buffer, texture_bid, transform_bg, rid: 0 });
    }
} 
