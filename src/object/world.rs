use rand::Rng;
use rand::thread_rng;
use std::collections::HashMap;
use wgpu::BindGroup;
use wgpu::BindGroupLayout;
use wgpu::Queue;
use wgpu::RenderPass;
use winit::dpi::PhysicalSize;

use crate::common::WithGPUBuffer;
use crate::object::{
    geometry::Geometry,
    scene::{Scene, generate_scene},
};

pub struct World {
    scene: Scene,
    geometries: HashMap<u32, Geometry>,
    uniform_bind_group: Option<BindGroup>,
}

impl World {
    pub fn new(screen_size: PhysicalSize<u32>) -> Self {
        Self {
            scene: generate_scene(screen_size),
            geometries: HashMap::new(),
            uniform_bind_group: None,
        }
    }
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.scene.resize(size);
    }

    pub fn add_geometry(&mut self, geo: Geometry) {
        let mut rng = thread_rng();
        let id: u32 = rng.r#gen();
        self.geometries.insert(id, geo);
    }

    pub fn move_obj(&mut self, move_info: [f32; 3]) {
        self.geometries.values_mut().for_each(|geo| {
            geo.move_obj(move_info);
        });
    }

    pub fn init_buffer(&mut self, device: &wgpu::Device, bind_group_layout: &BindGroupLayout) {
        self.scene.init_buffer(device);
        self.geometries.values_mut().for_each(|geo| {
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
                        .geometries
                        .values()
                        .last()
                        .as_ref()
                        .unwrap()
                        .position_buffer
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
        self.geometries.values_mut().for_each(|geo| {
            geo.update_buffer(queue);
        });
    }

    pub fn set_pipeline(&self, render_pass: &mut RenderPass) {
        let geometry = self.geometries.values().last().unwrap();
        let vertex_buffer = geometry.vertex_buffer.as_ref().unwrap();
        let vertex_data = &geometry.vertex_data;
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.draw(0..vertex_data.len() as u32, 0..1);
    }
}
