use std::{fs::File, io::BufReader};

use obj::load_obj;
use wgpu::{Buffer, Device, Queue};

pub type Color = [f32; 3];
pub type Position = [f32; 3];
pub type Normal = [f32; 3];
pub type Rotation = [f32; 3];
pub type Scale = [f32; 3];

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: Position,
    pub color: Color,
    pub normal: Normal,
}

unsafe impl bytemuck::Zeroable for Vertex {}

unsafe impl bytemuck::Pod for Vertex {}

pub trait WithGPUBuffer<const SIZE: usize> {
    fn init_buffer(&mut self, device: &Device) -> &Buffer;
    fn update_buffer(&mut self, queue: &Queue);
}
