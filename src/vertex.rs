#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    position: [f32; 3],
}

unsafe impl bytemuck::Zeroable for Vertex {}

unsafe impl bytemuck::Pod for Vertex {}

pub const VERTEX_LIST: &[Vertex] = &[
    // left
    Vertex {
        position: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [30.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 150.0, 0.0],
    },
    Vertex {
        position: [30.0, 150.0, 0.0],
    },
    //top
    Vertex {
        position: [30.0, 0.0, 0.0],
    },
    Vertex {
        position: [100.0, 0.0, 0.0],
    },
    Vertex {
        position: [30.0, 30.0, 0.0],
    },
    Vertex {
        position: [100.0, 30.0, 0.0],
    },
    //middle
    Vertex {
        position: [30.0, 60.0, 0.0],
    },
    Vertex {
        position: [70.0, 60.0, 0.0],
    },
    Vertex {
        position: [30.0, 90.0, 0.0],
    },
    Vertex {
        position: [70.0, 90.0, 0.0],
    },
    //left back
    Vertex {
        position: [0.0, 0.0, 30.0],
    },
    Vertex {
        position: [30.0, 0.0, 30.0],
    },
    Vertex {
        position: [0.0, 150.0, 30.0],
    },
    Vertex {
        position: [30.0, 150.0, 30.0],
    },
    //top back
    Vertex {
        position: [30.0, 0.0, 30.0],
    },
    Vertex {
        position: [100.0, 0.0, 30.0],
    },
    Vertex {
        position: [30.0, 30.0, 30.0],
    },
    Vertex {
        position: [100.0, 30.0, 30.0],
    },
    //middle back
    Vertex {
        position: [30.0, 60.0, 30.0],
    },
    Vertex {
        position: [70.0, 60.0, 30.0],
    },
    Vertex {
        position: [30.0, 90.0, 30.0],
    },
    Vertex {
        position: [70.0, 90.0, 30.0],
    },
];

pub const INDEX_LIST: &[u32] = &[
    // front
    0, 1, 2, 2, 1, 3, // left column
    4, 5, 6, 6, 5, 7, // top run
    8, 9, 10, 10, 9, 11, // middle run
    // back
    12, 13, 14, 14, 13, 15, // left column back
    16, 17, 18, 18, 17, 19, // top run back
    20, 21, 22, 22, 21, 23, // middle run back
    0, 5, 12, 12, 5, 17, // top
    5, 7, 17, 17, 7, 19, // top rung right
    6, 7, 18, 18, 7, 19, // top rung bottom
    6, 8, 18, 18, 8, 20, // between top and middle rung
    8, 9, 20, 20, 9, 21, // middle rung top
    9, 11, 21, 21, 11, 23, // middle rung right
    10, 11, 22, 22, 11, 23, // middle rung bottom
    10, 3, 22, 22, 3, 15, // stem right
    2, 3, 14, 14, 3, 15, // bottom
    0, 2, 12, 12, 2, 14, // left
];

pub fn create_vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
        array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[wgpu::VertexAttribute {
            offset: 0,
            shader_location: 0,
            format: wgpu::VertexFormat::Float32x3,
        }],
    }
}
