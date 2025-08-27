use std::sync::Arc;

use wgpu::{
    BufferAddress, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, FeaturesWGPU,
    FeaturesWebGPU, Instance, Limits, MemoryHints, PowerPreference, Queue, RequestAdapterOptions,
    Surface, SurfaceConfiguration, Trace, VertexAttribute, VertexBufferLayout, VertexFormat,
    VertexStepMode,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    content::{Vertex, world::World},
    render::render_config::RenderConfig,
};

pub struct WebGpuContext<'w> {
    pub device: Device,
    pub queue: Queue,
    surface: Surface<'w>,
    surface_config: SurfaceConfiguration,
    pub render_config: RenderConfig,
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

        let render_config = RenderConfig::new(&device, &surface_config);

        Self {
            surface,
            surface_config,
            render_config,
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
    pub fn draw(&mut self, world: &World) {
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next texture");
        //render pass
        {
            let mut render_pass = self
                .render_config
                .create_render_pass(&mut encoder, &surface_texture);
            world.set_pipeline(&mut render_pass)
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
