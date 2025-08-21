use bytemuck::cast_slice;
use wgpu::{
    Buffer, BufferUsages, Device,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    common::{Position, Rotation, Scale, Vertex, WithGPUBuffer},
    object::util::load_obj_model,
};

const SIZE: usize = 12;

pub struct ModelObject {
    pub vertex_data: Vec<Vertex>,
    pub vertex_buffer: Option<Buffer>,
    pub position_data: [f32; SIZE],
    pub position_buffer: Option<Buffer>,
}

impl ModelObject {
    pub fn new(path: &str, position_data: [f32; 12]) -> Self {
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
            position_data,
            position_buffer: None,
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

        self.position_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: cast_slice(&self.position_data),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        }));
        &self.position_buffer.as_ref().unwrap()
    }

    fn update_buffer(&mut self, queue: &wgpu::Queue) {
        queue.write_buffer(
            &self.position_buffer.as_ref().unwrap(),
            0,
            cast_slice(&self.position_data),
        );
    }
}

impl ModelObject {
    pub fn move_obj(&mut self, move_info: [f32; 3]) {
        self.position_data[8] += move_info[0];
        self.position_data[9] += move_info[1];
        self.position_data[10] += move_info[2];
    }

    pub fn rotate_obj(&mut self, rotate_info: [f32; 3]) {
        self.position_data[4] += rotate_info[0];
        self.position_data[5] += rotate_info[1];
        self.position_data[6] += rotate_info[2];
    }
}

pub fn generate_teapot() -> ModelObject {
    const PATH: &str = "src/object/asset/teapot.obj";
    // position info
    const DEFAULT_SCALE: Scale = [100.0, 100.0, 100.0];
    const DEFAULT_ROTATION: Rotation = [-90.0, 90.0, 0.0];
    const DEFAULT_POSITION: Position = [0.0, -100.0, -1500.0];

    ModelObject::new(
        PATH,
        [
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
        ],
    )
}
