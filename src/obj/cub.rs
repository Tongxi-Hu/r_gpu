use wgpu::util::DeviceExt;

use crate::common::{Color, Normal, Position, Rotation};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    position: Position,
    color: Color,
    normal: Normal,
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

pub const NORMAL: &[Normal] = &[
    [0.0, 0.0, 1.0],
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [-1.0, 0.0, 0.0],
    [0.0, -1.0, 0.0],
    [0.0, 0.0, -1.0],
];

pub const INDEX: &[u32] = &[
    0, 1, 2, 2, 1, 3, // front
    2, 3, 4, 4, 3, 5, // right
    0, 2, 4, 0, 4, 6, // top
    0, 7, 1, 0, 6, 7, // left
    1, 7, 5, 1, 5, 3, // bottom
    4, 5, 7, 4, 7, 6, //back
];

// position info
pub const DEFAULT_ROTATION: Rotation = [45.0, 45.0, 0.0];
pub const DEFAULT_POSITION: Position = [0., 0.0, -1300.0];

pub fn generate_vertex(
    position: &[Position],
    color: &[Color],
    normal: &[Normal],
    index: &[u32],
) -> Vec<Vertex> {
    let mut vertex = vec![];
    for (index, &v) in index.iter().enumerate() {
        vertex.push(Vertex {
            position: position[v as usize],
            color: color[index / 6],
            normal: normal[index / 6],
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
            wgpu::VertexAttribute {
                offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                shader_location: 2,
                format: wgpu::VertexFormat::Float32x3,
            },
        ],
    }
}

pub fn generate_position_buffer(
    rotation: Rotation,
    position: Position,
    device: &wgpu::Device,
) -> wgpu::Buffer {
    let data: [f32; 8] = [
        rotation[0],
        rotation[1],
        rotation[2],
        0.0,
        position[0],
        position[1],
        position[2],
        0.0,
    ];

    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}
