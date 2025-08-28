use wgpu::{
    BindGroupLayout, CommandEncoder, CompareFunction, DepthBiasState, DepthStencilState, Device,
    Extent3d, Face, LoadOp, MultisampleState, Operations, PipelineLayout, PipelineLayoutDescriptor,
    PrimitiveState, PrimitiveTopology, RenderPassDepthStencilAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, StencilState,
    StoreOp, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor, VertexState,
};

use crate::{
    constant::DEFAULT_SAMPLE_COUNT, content::world::World,
    render::web_gpu::create_vertex_buffer_layout,
};

pub struct ShadowConfig {
    pub shadow_pipeline: RenderPipeline,
    pub shadow_view: TextureView,
}

impl ShadowConfig {
    pub fn new(device: &Device, world_bind_layout: &(BindGroupLayout, BindGroupLayout)) -> Self {
        let shadow_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&world_bind_layout.0, &world_bind_layout.1],
            push_constant_ranges: &[],
        });

        let shadow_pipeline = create_shadow_pipeline(device, &shadow_pipeline_layout);

        let shadow_view = create_shadow_view(device);

        Self {
            shadow_pipeline,
            shadow_view,
        }
    }

    pub fn create_shadow_pass(&self, encoder: &mut CommandEncoder, world: &World) {
        let mut shadow_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &self.shadow_view,
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
        shadow_pass.set_pipeline(&self.shadow_pipeline);
        shadow_pass.set_bind_group(0, world.scene.scene_bind_group.as_ref().unwrap(), &[]);
        world.objects.values().for_each(|object| {
            shadow_pass.set_bind_group(1, object.transform_bind_group.as_ref().unwrap(), &[]);
            shadow_pass.set_vertex_buffer(0, object.vertex_buffer.as_ref().unwrap().slice(..));
            shadow_pass.draw(0..object.vertex_data.len() as u32, 0..1);
        });
    }
}

fn create_shadow_view(device: &Device) -> TextureView {
    device
        .create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width: 2048,
                height: 2048,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: DEFAULT_SAMPLE_COUNT,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        })
        .create_view(&TextureViewDescriptor::default())
}

fn create_shadow_pipeline(
    device: &Device,
    render_pipeline_layout: &PipelineLayout,
) -> RenderPipeline {
    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
            "shader/shadow.wgsl"
        ))),
    });
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(render_pipeline_layout),
        vertex: VertexState {
            module: &shader,
            entry_point: Some("shadow_main"),
            buffers: &[create_vertex_buffer_layout()],
            compilation_options: Default::default(),
        },
        fragment: None,
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            cull_mode: Some(Face::Back),
            ..Default::default()
        },
        depth_stencil: Some(DepthStencilState {
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
            format: TextureFormat::Depth32Float,
            bias: DepthBiasState::default(),
            stencil: StencilState::default(),
        }),
        multisample: MultisampleState {
            count: DEFAULT_SAMPLE_COUNT,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}
