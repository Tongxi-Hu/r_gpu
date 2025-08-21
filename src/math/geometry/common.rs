use crate::math::algebra::{point::Point, vector::Vector};

use super::ray::Ray;

pub type IntersectTargetID = u128;

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn norm_at(&self, point: &Point) -> Result<Vector, String>;
}

pub struct Intersection {
    t: f32,
    ray_direction: Vector,
    surface_point: Point,
    normal_v: Vector,
}

impl Intersection {
    pub fn new(t: f32, ray_direction: Vector, surface_point: Point, normal_v: Vector) -> Self {
        Self {
            t,
            ray_direction,
            surface_point,
            normal_v,
        }
    }

    pub fn get_t(&self) -> f32 {
        self.t
    }

    pub fn get_ray_direction(&self) -> Vector {
        self.ray_direction
    }

    pub fn get_normal(&self) -> Vector {
        self.normal_v
    }

    pub fn get_surface_point(&self) -> Point {
        self.surface_point
    }

    pub fn get_eye_v(&self) -> Vector {
        -self.ray_direction.unit().unwrap_or(Vector::unit_z())
    }
}
