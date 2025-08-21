use crate::math::algebra::{common::FuzzyEq, point::Point, vector::Vector};

use super::{
    common::{Intersect, Intersection},
    ray::Ray,
};

pub struct Triangle {
    p_0: Point,
    p_1: Point,
    p_2: Point,
}

impl Triangle {
    // triangle may be degenerate
    pub fn new(p_0: Point, p_1: Point, p_2: Point) -> Self {
        Self { p_0, p_1, p_2 }
    }

    pub fn area(&self) -> f32 {
        Vector::from_points(&self.p_0, &self.p_1)
            .cross(&Vector::from_points(&self.p_0, &self.p_2))
            .norm()
            / 2.0
    }

    pub fn norm(&self) -> Option<Vector> {
        Vector::from_points(&self.p_0, &self.p_1)
            .cross(&Vector::from_points(&self.p_0, &self.p_2))
            .unit()
            .ok()
    }

    pub fn is_on(&self, p: &Point) -> bool {
        if let Ok(norm) = self.norm_at(&self.p_2) {
            // check if the given point is the same as any of the vertices
            if *p == self.p_0 || *p == self.p_1 || *p == self.p_2 {
                return true;
            }
            let r_0 = Vector::from_points(&self.p_0, &p);
            // check if the given point in the same plane as the triangle
            if norm.dot(&r_0).fuzzy_eq(&0.0) {
                // check if then given point in the triangle
                let t_1 = Triangle::new(*p, self.p_0, self.p_1);
                let t_2 = Triangle::new(*p, self.p_1, self.p_2);
                let t_3 = Triangle::new(*p, self.p_2, self.p_0);
                if (t_1.area() + t_2.area() + t_3.area()).fuzzy_eq(&self.area()) {
                    return true;
                } else {
                    return false;
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl FuzzyEq for Triangle {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        self.p_0 == other.p_0 && self.p_1 == other.p_1 && self.p_2 == other.p_2
    }
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.fuzzy_eq(other)
    }
}

impl Intersect for Triangle {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersection: Vec<Intersection> = vec![];
        if let Some(norm) = &self.norm() {
            if ray.direction.dot(norm).fuzzy_eq(&0.0) {
                intersection
            } else {
                let a = norm.dot(&ray.origin.to_vector());
                let d = -(norm.dot(&self.p_0.to_vector()));
                let b = norm.dot(&ray.direction);
                let t = -(a + d) / b;
                let surface_point = ray.point_at(t);
                let normal_v_res = self.norm_at(&surface_point);
                if let (true, Ok(normal_v)) = (self.is_on(&surface_point), normal_v_res) {
                    intersection.push(Intersection::new(t, ray.direction, surface_point, normal_v));
                }
                intersection
            }
        } else {
            intersection
        }
    }

    fn norm_at(&self, _: &Point) -> Result<Vector, String> {
        self.norm().ok_or("invalid triangle".to_string())
    }
}
