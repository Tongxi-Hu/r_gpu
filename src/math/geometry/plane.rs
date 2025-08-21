use std::cmp;

use crate::math::algebra::{common::FuzzyEq, point::Point, vector::Vector};

use super::{
    common::{Intersect, Intersection},
    ray::Ray,
};

pub struct Plane {
    center: Point,
    norm: Vector,
    size: f32,
}

impl Plane {
    pub fn new(center: Point, norm: Vector, size: f32) -> Result<Self, String> {
        match norm.unit() {
            Err(string) => Err(string),
            Ok(unit) => Ok(Self {
                center,
                norm: unit,
                size,
            }),
        }
    }

    pub fn in_plane(&self, point: &Point) -> bool {
        self.norm
            .dot(&Vector::from_points(&self.center, &point))
            .fuzzy_eq(&0.0)
    }
}

impl Intersect for Plane {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersection: Vec<Intersection> = vec![];
        if ray.direction.dot(&self.norm).fuzzy_eq(&0.0) {
            intersection
        } else {
            let a = self.norm.dot(&ray.origin.to_vector());
            let d = -(self.norm.dot(&self.center.to_vector()));
            let b = self.norm.dot(&ray.direction);
            let t = -(a + d) / b;
            let surface_point = ray.point_at(t);
            let normal_v_res = self.norm_at(&surface_point);
            if let (cmp::Ordering::Less, Ok(normal_v)) = (
                surface_point.distance(&self.center).total_cmp(&self.size),
                normal_v_res,
            ) {
                intersection.push(Intersection::new(t, ray.direction, surface_point, normal_v));
            }
            intersection
        }
    }

    fn norm_at(&self, point: &Point) -> Result<Vector, String> {
        if self.in_plane(point) {
            Ok(self.norm)
        } else {
            Err("Point not in the plane".to_string())
        }
    }
}
