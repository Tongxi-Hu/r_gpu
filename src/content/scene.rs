use winit::dpi::PhysicalSize;

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
}

impl Scene {
    fn new(data: [f32; SCENE_SIZE]) -> Self {
        Self { scene_data: data }
    }
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.scene_data[0] = size.width as f32;
        self.scene_data[1] = size.height as f32;
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
