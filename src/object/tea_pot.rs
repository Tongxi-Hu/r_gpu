use wgpu::util::DeviceExt;

use crate::common::{Position, Rotation, Scale, Vertex, load_obj_model};

// position info
pub const DEFAULT_SCALE: Scale = [100.0, 100.0, 100.0];
pub const DEFAULT_ROTATION: Rotation = [0.0, 0.0, 90.0];
pub const DEFAULT_POSITION: Position = [0.0, 0.0, -1500.0];

pub fn generate_teapot_vertex() -> (u32, Vec<Vertex>) {
    let model = load_obj_model("src/object/asset/teapot.obj").unwrap();
    let mut vertex = vec![];
    for i in model.indices {
        let position = model.vertices[i as usize].position;
        vertex.push(Vertex {
            position,
            color: [0.0, 1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
        });
    }

    (vertex.len() as u32, vertex)
}

pub fn generate_teapot_position(device: &wgpu::Device) -> wgpu::Buffer {
    let data: [f32; 12] = [
        DEFAULT_SCALE[0],
        DEFAULT_SCALE[1],
        DEFAULT_SCALE[2],
        0.0,
        DEFAULT_ROTATION[0],
        DEFAULT_ROTATION[1],
        DEFAULT_ROTATION[2],
        0.0,
        DEFAULT_POSITION[0],
        DEFAULT_POSITION[1],
        DEFAULT_POSITION[2],
        0.0,
    ];

    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}

pub fn generate_teapot_vertex_position(device: &wgpu::Device) -> (u32, wgpu::Buffer, wgpu::Buffer) {
    let vertex_data = generate_teapot_vertex();
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&vertex_data.1),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let position_buffer = generate_teapot_position(device);
    (vertex_data.0, vertex_buffer, position_buffer)
}
