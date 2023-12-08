use std::num::NonZeroU32;

use glam::Mat3;
use glfw::Window;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt, RenderEncoder},
    *,
};

use crate::{RawVertex, RenderObject, RenderQuery, Vertex, BindGroupStorage};

pub struct RenderingServer {
    pub instance: Instance,
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub pipeline: RenderPipeline,
    pub texture_layout: BindGroupLayout,
    pub transform_layout: BindGroupLayout,
    pub texture_storage: BindGroupStorage,
    pub query: RenderQuery,
    pub window_size: (i32, i32),
}

impl RenderingServer {
    pub async fn new(window: &Window) -> Self {
        let size = window.get_size();
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::VULKAN,
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(window) }.unwrap();
        let adapter = create_adapter(&instance, &surface).await;
        let (device, queue) = request_device(&adapter).await;
        let shader = device.create_shader_module(include_wgsl!("../shaders/shader.wgsl"));
        let texture_layout = create_texture_bindgroup_layout(&device);
        let transform_layout = create_transform_bindgroup_layout(&device);

        let swapchain_caps = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(swapchain_caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.0 as u32,
            height: size.1 as u32,
            present_mode: PresentMode::AutoNoVsync,
            alpha_mode: swapchain_caps.alpha_modes[0],
            view_formats: vec![swapchain_format.add_srgb_suffix()],
        };

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&texture_layout, &transform_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[RawVertex::desc()],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: config.format,

                    blend: Some(BlendState {
                        color: BlendComponent {
                            src_factor: BlendFactor::SrcAlpha,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                        alpha: BlendComponent::OVER,
                    }),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });

        surface.configure(&device, &config);

        let query = RenderQuery::new();
        let window_size = window.get_size();

        let texture_storage = BindGroupStorage::new();

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            config,
            pipeline,
            texture_layout,
            transform_layout,
            texture_storage,
            query,
            window_size,
        }
    }

    pub fn reconfigure(&mut self, new_size: (i32, i32)) {
        self.window_size = new_size;
        self.config.width = new_size.0 as u32;
        self.config.height = new_size.1 as u32;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn push(&mut self, object: RenderObject) {
        self.query.push(object);
    }

    pub fn append(&mut self, objects: &mut Vec<RenderObject>) {
        self.query.append(objects);
    }

    pub fn draw(&mut self) -> Result<(), SurfaceError> {
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::WHITE),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            rpass.set_pipeline(&self.pipeline);

            let render_frame = RenderFrame { rpass: rpass };

            self.query.draw(&self, render_frame);
            self.query.clear();
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}

async fn create_adapter(instance: &Instance, surface: &Surface) -> Adapter {
    instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter")
}

async fn request_device(adapter: &Adapter) -> (Device, Queue) {
    let mut limits = Limits::default().using_resolution(adapter.limits());

    adapter
        .request_device(
            &DeviceDescriptor {
                label: None,
                features: Features::empty(),
                limits: limits,
            },
            None,
        )
        .await
        .expect("Failed to create device")
}

fn create_texture_bindgroup_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    multisampled: false,
                    view_dimension: TextureViewDimension::D2,
                    sample_type: TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            },
        ],
        label: None,
    })
}

fn create_transform_bindgroup_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: None,
    })
}

pub struct RenderFrame<'a> {
    pub rpass: RenderPass<'a>,
}
