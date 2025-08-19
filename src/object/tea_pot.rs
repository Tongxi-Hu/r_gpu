use std::fs;

use wavefront_obj::obj;
use wgpu::util::DeviceExt;

use crate::common::{Position, Rotation, Scale, Vertex};

// position info
pub const DEFAULT_SCALE: Scale = [100.0, 100.0, 100.0];
pub const DEFAULT_ROTATION: Rotation = [0.0, 0.0, 0.0];
pub const DEFAULT_POSITION: Position = [0., 0.0, -1300.0];

pub fn generate_teapot_vertex() -> (u32, Vec<Vertex>) {
    let content = fs::read_to_string("src/object/asset/teapot.obj").unwrap();
    let result = obj::parse(content).unwrap();
    let mut vertex = vec![];
    let count = result.objects[0].vertices.len();
    for (i, v) in result.objects[0].vertices.iter().enumerate() {
        let normal = result.objects[0].normals[i / 3];
        vertex.push(Vertex {
            position: [v.x as f32, v.y as f32, v.z as f32],
            color: [0.0, 1.0, 0.0],
            normal: [normal.x as f32, normal.y as f32, normal.z as f32],
        });
    }
    (count as u32, vertex)
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
