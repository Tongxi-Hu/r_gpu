type Color = [f32; 3];
type Position = [f32; 3];

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    position: Position,
    color: Color,
}

unsafe impl bytemuck::Zeroable for Vertex {}

unsafe impl bytemuck::Pod for Vertex {}

pub const POSITION: &[Position] = &[
    // left column
    [0.0, 0.0, 0.0],
    [30.0, 0.0, 0.0],
    [0.0, 150.0, 0.0],
    [30.0, 150.0, 0.0],
    // top rung
    [30.0, 0.0, 0.0],
    [100.0, 0.0, 0.0],
    [30.0, 30.0, 0.0],
    [100.0, 30.0, 0.0],
    // middle
    [30.0, 60.0, 0.0],
    [70.0, 60.0, 0.0],
    [30.0, 90.0, 0.0],
    [70.0, 90.0, 0.0],
    // left back
    [0.0, 0.0, 30.0],
    [30.0, 0.0, 30.0],
    [0.0, 150.0, 30.0],
    [30.0, 150.0, 30.0],
    // top back
    [30.0, 0.0, 30.0],
    [100.0, 0.0, 30.0],
    [30.0, 30.0, 30.0],
    [100.0, 30.0, 30.0],
    // middle back
    [30.0, 60.0, 30.0],
    [70.0, 60.0, 30.0],
    [30.0, 90.0, 30.0],
    [70.0, 90.0, 30.0],
];

pub const COLOR: &[Color] = &[
    [1.0, 0.0, 0.0], // left column front
    [1.0, 0.0, 0.0], // top rung front
    [1.0, 0.0, 0.0], // middle rung front
    [0.0, 1.0, 0.0], // left column back
    [0.0, 1.0, 0.0], // top rung back
    [0.0, 1.0, 0.0], // middle rung back
    [0.0, 0.0, 1.0], // top
    [1.0, 1.0, 0.0], // top rung right
    [0.0, 0.0, 1.0], // top rung bottom
    [1.0, 1.0, 0.0], // between top and middle rung
    [0.0, 0.0, 1.0], // middle rung top
    [1.0, 1.0, 0.0], // middle rung right
    [0.0, 0.0, 1.0], // middle rung bottom
    [1.0, 1.0, 0.0], // stem right
    [0.0, 0.0, 1.0], // bottom
    [1.0, 1.0, 1.0], // left
];

pub const INDEX: &[u32] = &[
    // front
    0, 1, 2, 2, 1, 3, // left column
    4, 5, 6, 6, 5, 7, // top run
    8, 9, 10, 10, 9, 11, // middle run
    // back
    12, 14, 13, 14, 15, 13, // left column back
    16, 18, 17, 18, 19, 17, // top run back
    20, 22, 21, 22, 23, 21, // middle run back
    0, 12, 5, 12, 17, 5, // top
    5, 17, 7, 17, 19, 7, // top rung right
    6, 7, 18, 18, 7, 19, // top rung bottom
    6, 18, 8, 18, 20, 8, // between top and middle rung
    8, 20, 9, 20, 21, 9, // middle rung top
    9, 21, 11, 21, 23, 11, // middle rung right
    10, 11, 22, 22, 11, 23, // middle rung bottom
    10, 22, 3, 22, 15, 3, // stem right
    2, 3, 14, 14, 3, 15, // bottom
    0, 2, 12, 12, 2, 14, // left
];

pub fn generate_vertex(position: &[Position], color: &[Color], index: &[u32]) -> Vec<Vertex> {
    let mut vertex = vec![];
    for (index, &v) in index.iter().enumerate() {
        vertex.push(Vertex {
            position: position[v as usize],
            color: color[index / 6],
        });
    }
    vertex
}

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
        ],
    }
}
