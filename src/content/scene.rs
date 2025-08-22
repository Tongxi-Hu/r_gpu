use bytemuck::cast_slice;
use wgpu::{
    BindGroup, BindGroupLayout, Buffer, BufferUsages, Device, Queue,
    util::{BufferInitDescriptor, DeviceExt},
};
use winit::dpi::PhysicalSize;

use crate::content::WithGPUBuffer;

// light position
const DEFAULT_LIGHT_POSITION: [f32; 3] = [0.0, 2000.0, -2000.0];
// eye position
const DEFAULT_EYE_POSITION: [f32; 3] = [0.0, 0.0, 0.0];

// perspective info
const DEFAULT_NEAR: f32 = -1000.0;
const DEFAULT_FAR: f32 = -20000.0;

pub const SCENE_SIZE: usize = 12;
pub struct Scene {
    pub scene_data: [f32; SCENE_SIZE],
    pub scene_buffer: Option<Buffer>,
    pub scene_bind_group: Option<BindGroup>,
}

impl Scene {
    fn new(data: [f32; SCENE_SIZE]) -> Self {
        Self {
            scene_data: data,
            scene_bind_group: None,
            scene_buffer: None,
        }
    }
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.scene_data[0] = size.width as f32;
        self.scene_data[1] = size.height as f32;
    }
}

impl WithGPUBuffer for Scene {
    fn init_buffer(&mut self, device: &Device, bind_group_layout: &BindGroupLayout) {
        self.scene_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: cast_slice(&self.scene_data),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        }));

        self.scene_bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
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
            cast_slice(&self.scene_data),
        );
    }
}

pub fn generate_scene(size: PhysicalSize<u32>) -> Scene {
    let scene_data: [f32; 12] = [
        size.width as f32,
        size.height as f32,
        DEFAULT_NEAR, // near
        DEFAULT_FAR,  // far
        DEFAULT_LIGHT_POSITION[0],
        DEFAULT_LIGHT_POSITION[1],
        DEFAULT_LIGHT_POSITION[2],
        0.0, // parallel light
        DEFAULT_EYE_POSITION[0],
        DEFAULT_EYE_POSITION[1],
        DEFAULT_EYE_POSITION[2],
        0.0,
    ];
    Scene::new(scene_data)
}
