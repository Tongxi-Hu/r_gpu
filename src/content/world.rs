use bytemuck::cast_slice;
use rand::Rng;
use rand::thread_rng;
use std::collections::HashMap;
use wgpu::Buffer;
use wgpu::BufferUsages;
use wgpu::Queue;
use wgpu::RenderPass;
use wgpu::util::BufferInitDescriptor;
use wgpu::util::DeviceExt;
use wgpu::{BindGroup, BindGroupLayout};
use winit::dpi::PhysicalSize;

use crate::content::scene::SCENE_SIZE;
use crate::content::{
    WithGPUBuffer,
    model_object::ModelObject,
    scene::{Scene, generate_scene},
};
use crate::math::algebra::matrix::Matrix;

pub struct World {
    scene: Scene,
    objects: HashMap<u32, ModelObject>,
    scene_buffer: Option<Buffer>,
    transform_buffer: Option<Buffer>,
    uniform_bind_group: Option<BindGroup>,
}

impl World {
    pub fn new(screen_size: PhysicalSize<u32>) -> Self {
        Self {
            scene: generate_scene(screen_size),
            objects: HashMap::new(),
            scene_buffer: None,
            transform_buffer: None,
            uniform_bind_group: None,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.scene.resize(size);
    }

    pub fn add_object(&mut self, model: ModelObject) {
        let mut rng = thread_rng();
        let id: u32 = rng.r#gen();
        self.objects.insert(id, model);
    }

    pub fn move_obj(&mut self, translation: Matrix<4>) {
        self.objects.values_mut().for_each(|model| {
            model.move_obj(translation);
        });
    }

    pub fn rotate_obj(&mut self, rotation: Matrix<4>) {
        self.objects.values_mut().for_each(|geo| {
            geo.rotate_obj(rotation);
        });
    }

    pub fn init_buffer(&mut self, device: &wgpu::Device, bind_group_layout: &BindGroupLayout) {
        self.objects.values_mut().for_each(|obj| {
            obj.init_buffer(device);
        });

        self.scene_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: cast_slice(&[1.0_f32; SCENE_SIZE]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        }));
        self.transform_buffer = Some(device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: cast_slice(&[Matrix::<4>::identity(); 3]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        }));

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.scene_buffer.as_ref().unwrap().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: self.transform_buffer.as_ref().unwrap().as_entire_binding(),
                },
            ],
        });
        self.uniform_bind_group = Some(uniform_bind_group);
    }

    pub fn set_pipeline(&self, render_pass: &mut RenderPass, queue: &wgpu::Queue) {
        queue.write_buffer(
            self.scene_buffer.as_ref().unwrap(),
            0,
            cast_slice(&self.scene.scene_data),
        );

        self.objects.values().for_each(|object| {
            queue.write_buffer(
                self.transform_buffer.as_ref().unwrap(),
                0,
                cast_slice(&object.transform),
            );
            render_pass.set_bind_group(0, self.uniform_bind_group.as_ref().unwrap(), &[]);
            render_pass.set_vertex_buffer(0, object.vertex_buffer.as_ref().unwrap().slice(..));
            render_pass.draw(0..object.vertex_data.len() as u32, 0..1);
        });
    }
}
