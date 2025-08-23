use rand::Rng;
use rand::thread_rng;
use std::collections::HashMap;
use wgpu::BindGroupLayout;
use wgpu::Device;
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
}

impl World {
    pub fn new(screen_size: PhysicalSize<u32>) -> Self {
        Self {
            scene: generate_scene(screen_size),
            objects: HashMap::new(),
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

    pub fn set_pipeline(&self, render_pass: &mut RenderPass) {
        render_pass.set_bind_group(0, self.scene.scene_bind_group.as_ref().unwrap(), &[]);
        self.objects.values().for_each(|object| {
            render_pass.set_bind_group(1, object.transform_bind_group.as_ref().unwrap(), &[]);
            render_pass.set_vertex_buffer(0, object.vertex_buffer.as_ref().unwrap().slice(..));
            render_pass.draw(0..object.vertex_data.len() as u32, 0..1);
        });
    }
}

impl WithGPUBuffer for World {
    fn init_buffer(&mut self, device: &Device, bind_group_layout: &[BindGroupLayout]) {
        self.scene.init_buffer(device, &bind_group_layout[0..=0]);
        if bind_group_layout.len() == 2 {
            self.objects.values_mut().for_each(|obj| {
                obj.init_buffer(device, &bind_group_layout[1..=1]);
            });
        }
    }

    fn update_buffer(&mut self, queue: &Queue) {
        self.scene.update_buffer(queue);
        self.objects.values_mut().for_each(|obj| {
            obj.update_buffer(queue);
        });
    }
}
