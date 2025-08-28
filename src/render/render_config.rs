use wgpu::{
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, Color,
    CommandEncoder, CompareFunction, DepthBiasState, DepthStencilState, Device, Extent3d, Face,
    FilterMode, FragmentState, LoadOp, MultisampleState, Operations, PipelineLayout,
    PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, SamplerBindingType, SamplerDescriptor, ShaderModuleDescriptor,
    ShaderSource, ShaderStages, StencilState, StoreOp, SurfaceConfiguration, SurfaceTexture,
    TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages,
    TextureView, TextureViewDescriptor, TextureViewDimension, VertexState,
};

use crate::{
    constant::DEFAULT_MULTI_SAMPLE_COUNT, content::world::World,
    render::web_gpu::create_vertex_buffer_layout,
};

pub struct RenderConfig {
    pub render_pipeline: RenderPipeline,
    depth_view: TextureView,
    multi_sample_view: TextureView,
    shadow_bind_group: BindGroup,
}

impl RenderConfig {
    pub fn new(
        device: &Device,
        surface_config: &SurfaceConfiguration,
        world_bind_layout: &(BindGroupLayout, BindGroupLayout),
        shadow_view: &TextureView,
    ) -> Self {
        let shadow_bind_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        sample_type: TextureSampleType::Depth,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Comparison), // Changed from Comparison
                    count: None,
                },
            ],
        });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &world_bind_layout.0,
                &world_bind_layout.1,
                &shadow_bind_layout,
            ],
            push_constant_ranges: &[],
        });

        let render_pipeline =
            create_render_pipeline(device, surface_config.format, &render_pipeline_layout);

        let (depth_view, multi_sample_view) = create_render_view(device, surface_config);

        let shadow_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &shadow_bind_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(shadow_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&device.create_sampler(
                        &SamplerDescriptor {
                            label: None,
                            address_mode_u: AddressMode::ClampToEdge,
                            address_mode_v: AddressMode::ClampToEdge,
                            address_mode_w: AddressMode::ClampToEdge,
                            mag_filter: FilterMode::Linear,
                            min_filter: FilterMode::Linear,
                            mipmap_filter: FilterMode::Nearest,
                            compare: Some(CompareFunction::Less),
                            ..Default::default()
                        },
                    )),
                },
            ],
        });

        Self {
            render_pipeline,
            depth_view,
            multi_sample_view,
            shadow_bind_group,
        }
    }

    pub fn update_render_view(&mut self, device: &Device, surface_config: &SurfaceConfiguration) {
        let (depth_view, multi_sample_view) = create_render_view(device, surface_config);
        self.depth_view = depth_view;
        self.multi_sample_view = multi_sample_view;
    }

    pub fn create_render_pass(
        &self,
        encoder: &mut CommandEncoder,
        surface_texture: &SurfaceTexture,
        world: &World,
    ) {
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
        render_pass.set_bind_group(0, world.scene.scene_bind_group.as_ref().unwrap(), &[]);
        render_pass.set_bind_group(2, &self.shadow_bind_group, &[]);
        world.objects.values().for_each(|object| {
            render_pass.set_bind_group(1, object.transform_bind_group.as_ref().unwrap(), &[]);
            render_pass.set_vertex_buffer(0, object.vertex_buffer.as_ref().unwrap().slice(..));
            render_pass.draw(0..object.vertex_data.len() as u32, 0..1);
        });
    }
}

fn create_render_view(
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
                sample_count: DEFAULT_MULTI_SAMPLE_COUNT,
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
                sample_count: DEFAULT_MULTI_SAMPLE_COUNT,
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
            count: DEFAULT_MULTI_SAMPLE_COUNT,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}
