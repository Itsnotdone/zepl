use bytemuck::{Pod, Zeroable};
use glam::{vec2, vec3, Vec2, Vec3};
use scene::Transform2d;
use wgpu::{vertex_attr_array, BufferAddress, VertexBufferLayout, VertexStepMode, VertexFormat};

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub tex_coords: Vec2,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: vec3(x, y, z),
            tex_coords: vec2(0.0, 0.0),
        }
    }

    pub fn with_tex_coords(mut self, x: f32, y: f32) -> Self {
        self.tex_coords = vec2(x, y);
        self
    }

    pub fn into_raw(self) -> RawVertex {
        RawVertex {
            position: self.position.to_array(),
            tex_coords: self.tex_coords.to_array(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct RawVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl RawVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2,
    ];

    pub fn desc() -> VertexBufferLayout<'static> {
        let size = std::mem::size_of::<Self>();
        VertexBufferLayout {
            array_stride: size as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
