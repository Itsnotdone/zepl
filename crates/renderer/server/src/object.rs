use ecs::{Entity, Resources};

use image::GenericImageView;
use scene::{Sprite, Transform2d};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    *,
};

use crate::{RawVertex, RenderFrame, RenderingServer, Vertex};

pub struct RenderObject {
    pub v_len: u32,
    pub v_buffer: Buffer,
    pub i_len: u32,
    pub i_buffer: Buffer,
    pub texture_bid: u32,
    pub transform_bg: BindGroup,
    pub rid: RID,
}

impl RenderObject {
    pub fn draw<'a>(&'a self, server: &'a RenderingServer, frame: &mut RenderFrame<'a>) {
        frame.rpass.set_bind_group(0, &server.texture_storage.get(self.texture_bid), &[]);
        frame
            .rpass
            .set_bind_group(1, &self.transform_bg, &[]);
        frame.rpass.set_vertex_buffer(0, self.v_buffer.slice(..));
        frame
            .rpass
            .set_index_buffer(self.i_buffer.slice(..), IndexFormat::Uint16);
        frame.rpass.draw_indexed(0..self.i_len, 0, 0..1);
    }
}

/// Render ID
pub type RID = u32;