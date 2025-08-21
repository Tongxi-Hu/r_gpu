use rand::Rng;
use rand::thread_rng;
use std::collections::HashMap;
use wgpu::BindGroup;
use wgpu::BindGroupLayout;
use wgpu::Queue;
use wgpu::RenderPass;
use winit::dpi::PhysicalSize;

use crate::content::{
    WithGPUBuffer,
    model_object::ModelObject,
    scene::{Scene, generate_scene},
};
use crate::math::algebra::matrix::Matrix;

pub struct World {
    scene: Scene,
    objects: HashMap<u32, ModelObject>,
    uniform_bind_group: Option<BindGroup>,
}

impl World {
    pub fn new(screen_size: PhysicalSize<u32>) -> Self {
        Self {
            scene: generate_scene(screen_size),
            objects: HashMap::new(),
            uniform_bind_group: None,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.scene.resize(size);
    }

    pub fn add_geometry(&mut self, model: ModelObject) {
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
        self.scene.init_buffer(device);
        self.objects.values_mut().for_each(|geo| {
            geo.init_buffer(device);
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.scene.buffer.as_ref().unwrap().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: self
                        .objects
                        .values()
                        .last()
                        .as_ref()
                        .unwrap()
                        .uniform_buffer
                        .as_ref()
                        .unwrap()
                        .as_entire_binding(),
                },
            ],
        });
        self.uniform_bind_group = Some(uniform_bind_group);
    }

    pub fn update_buffer(&mut self, queue: &Queue) {
        self.scene.update_buffer(queue);
        self.objects.values_mut().for_each(|geo| {
            geo.update_buffer(queue);
        });
    }

    pub fn set_pipeline(&self, render_pass: &mut RenderPass) {
        let object = self.objects.values().last().unwrap();
        let vertex_buffer = object.vertex_buffer.as_ref().unwrap();
        let vertex_data = &object.vertex_data;
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.draw(0..vertex_data.len() as u32, 0..1);
    }
}
