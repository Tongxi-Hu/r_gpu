#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    position: [f32; 2],
}

unsafe impl bytemuck::Zeroable for Vertex {}

unsafe impl bytemuck::Pod for Vertex {}

pub const VERTEX_LIST: &[Vertex] = &[
    // left
    Vertex {
        position: [0.0, 0.0],
    },
    Vertex {
        position: [30.0, 0.0],
    },
    Vertex {
        position: [0.0, 150.0],
    },
    Vertex {
        position: [30.0, 150.0],
    },
    //top
    Vertex {
        position: [30.0, 0.0],
    },
    Vertex {
        position: [100.0, 0.0],
    },
    Vertex {
        position: [30.0, 30.0],
    },
    Vertex {
        position: [100.0, 30.0],
    },
    //middle
    Vertex {
        position: [30.0, 60.0],
    },
    Vertex {
        position: [70.0, 60.0],
    },
    Vertex {
        position: [30.0, 90.0],
    },
    Vertex {
        position: [70.0, 90.0],
    },
];

pub const INDEX_LIST: &[u16] = &[
    0, 1, 2, 2, 1, 3, // left column
    4, 5, 6, 6, 5, 7, // top run
    8, 9, 10, 10, 9, 11, // middle run
];

pub fn create_vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[wgpu::VertexAttribute {
            offset: 0,
            shader_location: 0,
            format: wgpu::VertexFormat::Float32x2,
        }],
    }
}
