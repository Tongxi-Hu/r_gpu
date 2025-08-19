use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

// light position
const DEFAULT_LIGHT_POSITION: [f32; 3] = [0.0, 0.0, 0.0];
// eye position
const DEFAULT_EYE_POSITION: [f32; 3] = [0.0, 0.0, 0.0];

// perspective info
const DEFAULT_NEAR: f32 = -1000.0;
const DEFAULT_FAR: f32 = -20000.0;

pub fn generate_scene_buffer(
    size: PhysicalSize<u32>,
    device: &wgpu::Device,
) -> ([f32; 12], wgpu::Buffer) {
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
    (
        scene_data,
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&scene_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }),
    )
}
