use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, Color, CommandEncoder, CompareFunction, DepthBiasState, DepthStencilState,
    Device, Extent3d, Face, FragmentState, LoadOp, MultisampleState, Operations, PipelineLayout,
    PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology, RenderPass,
    RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages,
    StencilState, StoreOp, SurfaceConfiguration, SurfaceTexture, TextureDescriptor,
    TextureDimension, TextureFormat, TextureUsages, TextureView, TextureViewDescriptor,
    VertexState, wgc::id::markers::RenderPassEncoder,
};

use crate::render::web_gpu::create_vertex_buffer_layout;

const DEFAULT_MULTI_SAMPLE: u32 = 4;

pub struct RenderConfig {
    pub render_pipeline: RenderPipeline,
    pub bind_group_layout: [BindGroupLayout; 2],
    depth_view: TextureView,
    multi_sample_view: TextureView,
}

impl RenderConfig {
    pub fn new(device: &Device, surface_config: &SurfaceConfiguration) -> Self {
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
            create_render_pipeline(device, surface_config.format, &render_pipeline_layout);

        let (depth_view, multi_sample_view) = create_render_view(device, surface_config);

        Self {
            render_pipeline,
            bind_group_layout: [scene_bind_layout, model_bind_layout],
            depth_view,
            multi_sample_view,
        }
    }

    pub fn update_render_view(&mut self, device: &Device, surface_config: &SurfaceConfiguration) {
        let (depth_view, multi_sample_view) = create_render_view(device, surface_config);
        self.depth_view = depth_view;
        self.multi_sample_view = multi_sample_view;
    }

    pub fn create_render_pass<'a, 'b: 'a>(
        &'a self,
        encoder: &'b mut CommandEncoder,
        surface_texture: &SurfaceTexture,
    ) -> RenderPass<'a> {
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &self.multi_sample_view,
                resolve_target: Some(
                    &surface_texture
                        .texture
                        .create_view(&TextureViewDescriptor::default()),
                ),
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &self.depth_view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: Some(Operations {
                    load: LoadOp::Clear(0),
                    store: StoreOp::Store,
                }),
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_pipeline(&self.render_pipeline);
        return render_pass;
    }
}

pub fn create_render_view(
    device: &Device,
    surface_config: &SurfaceConfiguration,
) -> (TextureView, TextureView) {
    (
        device
            .create_texture(&TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: surface_config.width.max(1),
                    height: surface_config.height.max(1),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: DEFAULT_MULTI_SAMPLE,
                dimension: TextureDimension::D2,
                format: TextureFormat::Depth32FloatStencil8,
                usage: TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[TextureFormat::Depth32FloatStencil8],
            })
            .create_view(&TextureViewDescriptor::default()),
        device
            .create_texture(&TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: surface_config.width.max(1),
                    height: surface_config.height.max(1),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: DEFAULT_MULTI_SAMPLE,
                dimension: TextureDimension::D2,
                format: surface_config.format,
                usage: TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[surface_config.format],
            })
            .create_view(&TextureViewDescriptor::default()),
    )
}

fn create_render_pipeline(
    device: &Device,
    swap_chain_format: TextureFormat,
    render_pipeline_layout: &PipelineLayout,
) -> RenderPipeline {
    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
            "shader/object.wgsl"
        ))),
    });
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(render_pipeline_layout),
        vertex: VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[create_vertex_buffer_layout()],
            compilation_options: Default::default(),
        },
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(swap_chain_format.into())],
        }),
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            cull_mode: Some(Face::Back),
            ..Default::default()
        },
        depth_stencil: Some(DepthStencilState {
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
            format: TextureFormat::Depth32FloatStencil8,
            bias: DepthBiasState::default(),
            stencil: StencilState::default(),
        }),
        multisample: MultisampleState {
            count: DEFAULT_MULTI_SAMPLE,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}
