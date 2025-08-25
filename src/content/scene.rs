use bytemuck::cast_slice;
use wgpu::{
    BindGroup, BindGroupLayout, Buffer, BufferUsages, Device, Queue,
    util::{BufferInitDescriptor, DeviceExt},
};
use winit::dpi::PhysicalSize;

use crate::{
    content::WithGPUBuffer,
    math::algebra::{common::Dimension4, point::Point},
};

pub struct Scene {
    pub scene_buffer: Option<Buffer>,
    pub scene_bind_group: Option<BindGroup>,

    // (width, height, near, far)
    scene_config: Point,
    light_position: Point,
    light_direction: Point,
    eye_position: Point,
}

impl Scene {
    fn new(
        scene_config: Point,
        light_position: Point,
        light_direction: Point,
        eye_position: Point,
    ) -> Self {
        Self {
            scene_config,
            light_position,
            light_direction,
            eye_position,
            scene_bind_group: None,
            scene_buffer: None,
        }
    }
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.scene_config.set_x(size.width as f32);
        self.scene_config.set_y(size.height as f32);
    }
}

impl WithGPUBuffer for Scene {
    fn init_buffer(&mut self, device: &Device, bind_group_layout: &[BindGroupLayout]) {
        self.scene_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: cast_slice(&[
                self.scene_config,
                self.light_position,
                self.light_direction,
                self.eye_position,
            ]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        }));

        self.scene_bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout[0],
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.scene_buffer.as_ref().unwrap().as_entire_binding(),
            }],
        }));
    }

    fn update_buffer(&mut self, queue: &Queue) {
        queue.write_buffer(
            self.scene_buffer.as_ref().unwrap(),
            0,
            cast_slice(&[
                self.scene_config,
                self.light_position,
                self.light_direction,
                self.eye_position,
            ]),
        );
    }
}

pub fn generate_scene(size: PhysicalSize<u32>) -> Scene {
    // perspective
    let near: f32 = -1000.0;
    let far: f32 = -20000.0;
    // light
    let light_position: [f32; 3] = [0.0, 500.0, -2000.0];
    let light_direction: [f32; 3] = [0.0, 0.0, -1.0];
    // eye
    let eye_position: [f32; 3] = [0.0, 0.0, 0.0];

    Scene::new(
        Point::new(size.width as f32, size.height as f32, near, far),
        Point::point(light_position[0], light_position[1], light_position[2]),
        Point::point(light_direction[0], light_direction[1], light_direction[2]),
        Point::point(eye_position[0], eye_position[1], eye_position[2]),
    )
}
