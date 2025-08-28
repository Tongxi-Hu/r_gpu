use std::sync::Arc;

use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferAddress,
    BufferBindingType, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, FeaturesWGPU,
    FeaturesWebGPU, Instance, Limits, MemoryHints, PowerPreference, Queue, RequestAdapterOptions,
    ShaderStages, Surface, SurfaceConfiguration, Trace, VertexAttribute, VertexBufferLayout,
    VertexFormat, VertexStepMode,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    content::{Vertex, WithGPUBuffer, world::World},
    render::{render_config::RenderConfig, shadow_config::ShadowConfig},
};

pub struct WebGpuContext<'w> {
    pub device: Device,
    pub queue: Queue,
    surface: Surface<'w>,
    surface_config: SurfaceConfiguration,
    world_bind_layout: (BindGroupLayout, BindGroupLayout),
    pub render_config: RenderConfig,
    pub shadow_config: ShadowConfig,
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

        let world_bind_layout = (
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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
            }),
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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
            }),
        );

        let shadow_config = ShadowConfig::new(&device, &world_bind_layout);
        let render_config = RenderConfig::new(
            &device,
            &surface_config,
            &world_bind_layout,
            &shadow_config.shadow_view,
        );

        Self {
            surface,
            surface_config,
            world_bind_layout,
            render_config,
            shadow_config,
            device,
            queue,
        }
    }

    pub fn new(window: Arc<Window>) -> Self {
        pollster::block_on(WebGpuContext::new_async(window))
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.surface_config.width = size.width.max(1);
        self.surface_config.height = size.height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
        self.render_config
            .update_render_view(&self.device, &self.surface_config);
    }

    pub fn init_buffer(&self, world: &mut World) {
        world
            .scene
            .init_buffer(&self.device, &self.world_bind_layout.0);
        world.objects.values_mut().for_each(|obj| {
            obj.init_buffer(&self.device, &self.world_bind_layout.1);
        });
    }

    pub fn update_buffer(&self, world: &mut World) {
        world.scene.update_buffer(&self.queue);
        world.objects.values_mut().for_each(|obj| {
            obj.update_buffer(&self.queue);
        });
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
            self.shadow_config.create_shadow_pass(&mut encoder, &world);
        }

        //render pass
        {
            self.render_config
                .create_render_pass(&mut encoder, &surface_texture, &world);
        }

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}

pub fn create_vertex_buffer_layout() -> VertexBufferLayout<'static> {
    VertexBufferLayout {
        array_stride: size_of::<Vertex>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: &[
            VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: VertexFormat::Float32x4,
            },
            VertexAttribute {
                offset: size_of::<[f32; 4]>() as BufferAddress,
                shader_location: 1,
                format: VertexFormat::Float32x4,
            },
            VertexAttribute {
                offset: size_of::<[f32; 8]>() as BufferAddress,
                shader_location: 2,
                format: VertexFormat::Float32x4,
            },
        ],
    }
}
