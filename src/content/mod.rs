use wgpu::{BindGroupLayout, Device, Queue};

use crate::{
    math::algebra::{point::Point, vector::Vector},
    physics::color::Color,
};

pub mod model_object;
pub mod scene;
pub mod world;

pub trait WithGPUBuffer {
    fn init_buffer(&mut self, device: &Device, bind_group_layout: &BindGroupLayout);
    fn update_buffer(&mut self, queue: &Queue);
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: Point,
    pub color: Color,
    pub normal: Vector,
}

unsafe impl bytemuck::Zeroable for Vertex {}

unsafe impl bytemuck::Pod for Vertex {}
