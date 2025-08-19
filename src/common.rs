pub type Color = [f32; 3];
pub type Position = [f32; 3];
pub type Normal = [f32; 3];
pub type Rotation = [f32; 3];
pub type Scale = [f32; 3];

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: Position,
    pub color: Color,
    pub normal: Normal,
}

unsafe impl bytemuck::Zeroable for Vertex {}

unsafe impl bytemuck::Pod for Vertex {}

pub fn create_vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x3,
            },
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                shader_location: 2,
                format: wgpu::VertexFormat::Float32x3,
            },
        ],
    }
}
