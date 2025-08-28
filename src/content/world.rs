use rand::Rng;
use rand::thread_rng;
use std::collections::HashMap;
use winit::dpi::PhysicalSize;

use crate::content::{
    model_object::ModelObject,
    scene::{Scene, generate_scene},
};
use crate::math::algebra::matrix::Matrix;

pub struct World {
    pub scene: Scene,
    pub objects: HashMap<u32, ModelObject>,
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
}
