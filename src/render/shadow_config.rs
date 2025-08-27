use wgpu::{BindGroupLayout, RenderPipeline, TextureView};

pub struct ShadowConfig {
    pub render_pipeline: RenderPipeline,
    pub bind_group_layout: [BindGroupLayout; 2],
    shadow_view: TextureView,
}
