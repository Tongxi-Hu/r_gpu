use crate::math::algebra::{point::Point, vector::Vector};

use super::{
    common::{Intersect, Intersection},
    triangle::Triangle,
};

pub struct Polyhedron {
    triangles: Vec<Triangle>,
}

impl Polyhedron {
    pub fn new(vertices: Vec<Point>, surfaces: Vec<(usize, usize, usize)>) -> Self {
        let mut triangles = vec![];
        surfaces.iter().for_each(|surface| {
            triangles.push(Triangle::new(
                vertices[surface.0],
                vertices[surface.1],
                vertices[surface.2],
            ))
        });
        Self { triangles }
    }

    pub fn norm_to(&self, surface_index: usize) -> Option<Vector> {
        self.triangles
            .get(surface_index)
            .and_then(|triangle| triangle.norm())
    }

    pub fn on_surface(&self, point: &Point) -> Option<usize> {
        self.triangles
            .iter()
            .enumerate()
            .find(|(_, triangle)| triangle.is_on(point))
            .and_then(|item| Some(item.0))
    }
}

impl Intersect for Polyhedron {
    fn intersect(&self, ray: &super::ray::Ray) -> Vec<Intersection> {
        let mut intersection = vec![];
        self.triangles.iter().for_each(|triangle| {
            intersection.append(&mut triangle.intersect(ray));
        });
        //only take the first intersection with minimum t
        if intersection.len() > 1 {
            if let Some(min) = intersection
                .into_iter()
                .min_by(|a, b| a.get_t().total_cmp(&b.get_t()))
            {
                return vec![min];
            } else {
                return vec![];
            }
        } else {
            intersection
        }
    }

    fn norm_at(&self, point: &Point) -> Result<Vector, String> {
        if let Some(surface_index) = self.on_surface(point) {
            self.norm_to(surface_index).ok_or("not found".to_string())
        } else {
            Err("point not on surface".to_string())
        }
    }
}
