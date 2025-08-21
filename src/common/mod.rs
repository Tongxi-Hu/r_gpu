use wgpu::{Buffer, Device, Queue};

pub trait WithGPUBuffer {
    fn init_buffer(&mut self, device: &Device) -> &Buffer;
    fn update_buffer(&mut self, queue: &Queue);
}
