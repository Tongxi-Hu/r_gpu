use std::sync::Arc;

use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, CommandEncoderDescriptor, Device, DeviceDescriptor, Extent3d, Face,
    Features, FeaturesWGPU, FeaturesWebGPU, Instance, Limits, MemoryHints,
    PipelineLayoutDescriptor, PowerPreference, Queue, RenderPipeline, RequestAdapterOptions,
    ShaderStages, Surface, SurfaceConfiguration, Texture, TextureDescriptor, TextureDimension,
    TextureFormat, TextureUsages, TextureView, Trace,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::content::{Vertex, world::World};

const DEFAULT_MULTI_SAMPLE: u32 = 4;

pub struct WebGpuContext<'w> {
    pub device: Device,
    pub queue: Queue,
    pub bind_group_layout: [BindGroupLayout; 2],

    surface: Surface<'w>,
    surface_config: SurfaceConfiguration,
    render_pipeline: RenderPipeline,
    depth_view: TextureView,
    color_view: TextureView,
    resolve_target_view: TextureView,
}

impl<'w> WebGpuContext<'w> {
    pub async fn new_async(window: Arc<Window>) -> Self {
        let instance = Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("fail to find adaptor");

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: None,
                required_features: Features {
                    features_webgpu: FeaturesWebGPU::DEPTH32FLOAT_STENCIL8,
                    features_wgpu: FeaturesWGPU::empty(),
                },
                required_limits: Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: MemoryHints::Performance,
                trace: Trace::Off,
            })
            .await
            .expect("fail to create device");

        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);
        let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
        surface.configure(&device, &surface_config);

        let depth_texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: surface_config.width.max(1),
                height: surface_config.height.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: DEFAULT_MULTI_SAMPLE,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32FloatStencil8,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[wgpu::TextureFormat::Depth32FloatStencil8],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let multi_sample_texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: surface_config.width.max(1),
                height: surface_config.height.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: DEFAULT_MULTI_SAMPLE,
            dimension: wgpu::TextureDimension::D2,
            format: surface_config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[surface_config.format],
        });

        let color_view = multi_sample_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let resolve_target_texture = surface
            .get_current_texture()
            .expect("Failed to acquire next texture");

        let resolve_target_view = resolve_target_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let scene_bind_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let model_bind_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&scene_bind_layout, &model_bind_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline =
            create_pipeline(&device, surface_config.format, &render_pipeline_layout);

        Self {
            surface,
            surface_config,
            device,
            queue,
            depth_view,
            color_view,
            resolve_target_view,
            render_pipeline,
            bind_group_layout: [scene_bind_layout, model_bind_layout],
        }
    }

    pub fn new(window: Arc<Window>) -> Self {
        pollster::block_on(WebGpuContext::new_async(window))
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.surface_config.width = size.width.max(1);
        self.surface_config.height = size.height.max(1);
        self.surface.configure(&self.device, &self.surface_config);

        self.depth_view = self
            .device
            .create_texture(&TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: self.surface_config.width.max(1),
                    height: self.surface_config.height.max(1),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: DEFAULT_MULTI_SAMPLE,
                dimension: TextureDimension::D2,
                format: TextureFormat::Depth32FloatStencil8,
                usage: TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[TextureFormat::Depth32FloatStencil8],
            })
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.color_view = self
            .device
            .create_texture(&TextureDescriptor {
                label: None,
                size: wgpu::Extent3d {
                    width: self.surface_config.width.max(1),
                    height: self.surface_config.height.max(1),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: DEFAULT_MULTI_SAMPLE,
                dimension: wgpu::TextureDimension::D2,
                format: self.surface_config.format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[self.surface_config.format],
            })
            .create_view(&wgpu::TextureViewDescriptor::default());
    }
    pub fn draw(&mut self, world: &World) {
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next texture");

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.color_view,
                    resolve_target: Some(
                        &surface_texture
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default()),
                    ),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0),
                        store: wgpu::StoreOp::Store,
                    }),
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            world.set_pipeline(&mut render_pass)
        }

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}

fn create_vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: size_of::<[f32; 4]>() as wgpu::BufferAddress,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: size_of::<[f32; 8]>() as wgpu::BufferAddress,
                shader_location: 2,
                format: wgpu::VertexFormat::Float32x4,
            },
        ],
    }
}

fn create_pipeline(
    device: &wgpu::Device,
    swap_chain_format: wgpu::TextureFormat,
    render_pipeline_layout: &wgpu::PipelineLayout,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
            "shader/basic.wgsl"
        ))),
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[create_vertex_buffer_layout()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(swap_chain_format.into())],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: Some(Face::Back),
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            format: wgpu::TextureFormat::Depth32FloatStencil8,
            bias: wgpu::DepthBiasState::default(),
            stencil: wgpu::StencilState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: DEFAULT_MULTI_SAMPLE,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}
