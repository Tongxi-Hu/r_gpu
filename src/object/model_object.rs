use std::{fs::File, io::BufReader};

use bytemuck::cast_slice;
use obj::load_obj;
use wgpu::{
    Buffer, BufferUsages, Device,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    common::{Position, Rotation, Scale, Vertex, WithGPUBuffer},
    math::algebra::matrix::Matrix,
};

const SIZE: usize = 12;

pub struct ModelObject {
    pub vertex_data: Vec<Vertex>,
    pub vertex_buffer: Option<Buffer>,
    transform: [Matrix<4>; 3],
    pub uniform_buffer: Option<Buffer>,
}

impl ModelObject {
    pub fn new(path: &str, scale: Matrix<4>, rotation: Matrix<4>, translation: Matrix<4>) -> Self {
        let model = load_obj_model(path).unwrap();
        let mut vertex_data = vec![];
        for i in model.indices {
            let position = model.vertices[i as usize].position;
            let normal = model.vertices[i as usize].normal;
            vertex_data.push(Vertex {
                position,
                color: [0.439, 0.329, 0.243],
                normal,
            });
        }
        Self {
            vertex_data,
            vertex_buffer: None,
            transform: [scale, rotation, translation],
            uniform_buffer: None,
        }
    }
}

impl WithGPUBuffer<SIZE> for ModelObject {
    fn init_buffer(&mut self, device: &Device) -> &Buffer {
        self.vertex_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: cast_slice(&self.vertex_data),
            usage: BufferUsages::VERTEX,
        }));

        self.uniform_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: cast_slice(&self.transform),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        }));
        &self.uniform_buffer.as_ref().unwrap()
    }

    fn update_buffer(&mut self, queue: &wgpu::Queue) {
        queue.write_buffer(
            &self.uniform_buffer.as_ref().unwrap(),
            0,
            cast_slice(&self.transform),
        );
    }
}

impl ModelObject {
    pub fn move_obj(&mut self, translation: Matrix<4>) {
        self.transform[2] = translation * self.transform[2];
    }

    pub fn rotate_obj(&mut self, rotation: Matrix<4>) {
        self.transform[1] = rotation * self.transform[1];
    }
}

pub fn load_obj_model(path: &str) -> Result<obj::Obj, Box<dyn std::error::Error>> {
    let buffer = BufReader::new(File::open(path)?);
    let model = load_obj(buffer)?;
    Ok(model)
}

pub fn generate_teapot() -> ModelObject {
    const PATH: &str = "src/object/asset/teapot.obj";
    // position info
    const DEFAULT_SCALE: Scale = [100.0, 100.0, 100.0];
    const DEFAULT_ROTATION: Rotation = [-90.0, 90.0, 0.0];
    const DEFAULT_POSITION: Position = [0.0, -100.0, -1500.0];

    ModelObject::new(
        PATH,
        Matrix::<4>::scale(DEFAULT_SCALE[0], DEFAULT_SCALE[1], DEFAULT_SCALE[2]),
        Matrix::<4>::rotate_z(DEFAULT_ROTATION[2])
            * Matrix::<4>::rotate_y(DEFAULT_ROTATION[1])
            * Matrix::<4>::rotate_x(DEFAULT_ROTATION[0]),
        Matrix::<4>::translation(
            DEFAULT_POSITION[0],
            DEFAULT_POSITION[1],
            DEFAULT_POSITION[2],
        ),
    )
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
