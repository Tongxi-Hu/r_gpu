use wgpu::{Buffer, Device, Queue};

pub mod model_object;
pub mod scene;
pub mod world;

pub trait WithGPUBuffer {
    fn init_buffer(&mut self, device: &Device) -> &Buffer;
    fn update_buffer(&mut self, queue: &Queue);
}
