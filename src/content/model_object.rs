use std::{fs::File, io::BufReader};

use bytemuck::cast_slice;
use obj::load_obj;
use wgpu::{
    BindGroup, BindGroupLayout, Buffer, BufferUsages, Device, Queue,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    content::{Vertex, WithGPUBuffer},
    math::algebra::{matrix::Matrix, point::Point, vector::Vector},
    physics::color::Color,
};

pub struct ModelObject {
    pub vertex_data: Vec<Vertex>,
    pub vertex_buffer: Option<Buffer>,
    pub transform: [Matrix<4>; 3],
    pub transform_buffer: Option<Buffer>,
    pub transform_bind_group: Option<BindGroup>,
}

impl ModelObject {
    pub fn new(
        vertex_data: Vec<Vertex>,
        scale: Matrix<4>,
        rotation: Matrix<4>,
        translation: Matrix<4>,
    ) -> Self {
        Self {
            vertex_data,
            vertex_buffer: None,
            transform: [scale, rotation, translation],
            transform_buffer: None,
            transform_bind_group: None,
        }
    }
}

impl WithGPUBuffer for ModelObject {
    fn init_buffer(&mut self, device: &Device, bind_group_layout: &BindGroupLayout) {
        self.vertex_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: cast_slice(&self.vertex_data),
            usage: BufferUsages::VERTEX,
        }));

        self.transform_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: cast_slice(&self.transform),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        }));

        self.transform_bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.transform_buffer.as_ref().unwrap().as_entire_binding(),
            }],
        }));
    }

    fn update_buffer(&mut self, queue: &Queue) {
        queue.write_buffer(
            self.transform_buffer.as_ref().unwrap(),
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
    const PATH: &str = "src/content/asset/teapot.obj";

    let model = load_obj_model(PATH).unwrap();
    let mut vertex_data = vec![];
    for i in model.indices {
        let position = model.vertices[i as usize].position;
        let normal = model.vertices[i as usize].normal;
        vertex_data.push(Vertex {
            position: Point::point(position[0], position[1], position[2]),
            color: Color::rgb(0.439, 0.329, 0.243),
            normal: Vector::vector(normal[0], normal[1], normal[2]),
        });
    }

    // position info
    let scale: [f32; 3] = [100.0, 100.0, 100.0];
    let rotation: [f32; 3] = [-90.0, 90.0, 0.0];
    let position: [f32; 3] = [0.0, -100.0, -2000.0];

    ModelObject::new(
        vertex_data,
        Matrix::<4>::scale(scale[0], scale[1], scale[2]),
        Matrix::<4>::rotate_z(rotation[2])
            * Matrix::<4>::rotate_y(rotation[1])
            * Matrix::<4>::rotate_x(rotation[0]),
        Matrix::<4>::translation(position[0], position[1], position[2]),
    )
}

pub fn generate_ground() -> ModelObject {
    let vertex_data: Vec<Vertex> = vec![
        Vertex {
            position: Point::point(5000.0, 0.0, 5000.0),
            color: Color::rgb(1.0, 1.0, 1.0),
            normal: Vector::unit_y(),
        },
        Vertex {
            position: Point::point(5000.0, 0.0, -5000.0),
            color: Color::rgb(1.0, 1.0, 1.0),
            normal: Vector::unit_y(),
        },
        Vertex {
            position: Point::point(-5000.0, 0.0, 5000.0),
            color: Color::rgb(1.0, 1.0, 1.0),
            normal: Vector::unit_y(),
        },
        Vertex {
            position: Point::point(-5000.0, 0.0, -5000.0),
            color: Color::rgb(1.0, 1.0, 1.0),
            normal: Vector::unit_y(),
        },
        Vertex {
            position: Point::point(-5000.0, 0.0, 5000.0),
            color: Color::rgb(1.0, 1.0, 1.0),
            normal: Vector::unit_y(),
        },
        Vertex {
            position: Point::point(5000.0, 0.0, -5000.0),
            color: Color::rgb(1.0, 1.0, 1.0),
            normal: Vector::unit_y(),
        },
    ];

    // position info
    let scale: [f32; 3] = [1.0, 1.0, 1.0];
    let rotation: [f32; 3] = [0.0, 0.0, 0.0];
    let position: [f32; 3] = [0.0, -1000.0, -2000.0];

    ModelObject::new(
        vertex_data,
        Matrix::<4>::scale(scale[0], scale[1], scale[2]),
        Matrix::<4>::rotate_z(rotation[2])
            * Matrix::<4>::rotate_y(rotation[1])
            * Matrix::<4>::rotate_x(rotation[0]),
        Matrix::<4>::translation(position[0], position[1], position[2]),
    )
}
