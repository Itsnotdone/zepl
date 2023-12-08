use scene::Texture;
use server::RenderingServer;

use wgpu::*;

pub(crate) trait TextureMethods{
    fn prepare(&mut self, server: &mut RenderingServer) -> u32;
    fn create_sampler(&self, server: &RenderingServer) -> Sampler;
    fn write_texture(&self, server: &RenderingServer, w_texture: &wgpu::Texture, extent: Extent3d);
}

impl TextureMethods for Texture{
    fn prepare(&mut self, server: &mut RenderingServer) -> u32{
        if let Some(bid) = self.get_bid(){
            return bid;
        }

        let extent = Extent3d{
            width: self.size.0,
            height: self.size.1,
            depth_or_array_layers: 1
        };

        let w_texture = server.device.create_texture(&TextureDescriptor {
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
            label: None,
        });

        self.write_texture(server, &w_texture, extent);        

        let view = w_texture.create_view(&TextureViewDescriptor::default());
        let sampler = self.create_sampler(server);

        let bind_group = server.device.create_bind_group(&BindGroupDescriptor {
            layout: &server.texture_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler)
                },
            ],
            label: None,
        });

        let bid = server.texture_storage.push(bind_group);
        self.set_bid(bid);

        bid

    }

    fn create_sampler(&self, server: &RenderingServer) -> Sampler{
        let sampler = server.device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            ..Default::default()
        });

        sampler
    }

    fn write_texture(&self, server: &RenderingServer, w_texture: &wgpu::Texture, extent: Extent3d){
        server.queue.write_texture(
            ImageCopyTexture {
                texture: &w_texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &self.data,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * self.size.0),
                rows_per_image: Some(self.size.1),
            },
            extent,
        );
    }
}