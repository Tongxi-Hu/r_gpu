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
    [-100.0, 100.0, 100.0],   //0
    [-100.0, -100.0, 100.0],  //1
    [100.0, 100.0, 100.0],    //2
    [100.0, -100.0, 100.0],   //3
    [100.0, 100.0, -100.0],   //4
    [100.0, -100.0, -100.0],  //5
    [-100.0, 100.0, -100.0],  //6
    [-100.0, -100.0, -100.0], //7
];

pub const COLOR: &[Color] = &[
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
    [1.0, 1.0, 0.0],
    [1.0, 0.0, 1.0],
    [0.0, 1.0, 1.0],
];

pub const INDEX: &[u32] = &[
    0, 1, 2, 2, 1, 3, // front
    2, 3, 4, 4, 3, 5, // right
    0, 2, 4, 0, 4, 6, // top
    0, 7, 1, 0, 6, 7, // left
    1, 7, 5, 1, 5, 3, // bottom
    4, 5, 7, 4, 7, 6, //back
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
