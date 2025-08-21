use crate::math::algebra::{common::FuzzyEq, point::Point, vector::Vector};

use super::{
    common::{Intersect, Intersection},
    ray::Ray,
};

pub struct Sphere {
    origin: Point,
    radius: f32,
}

impl Sphere {
    pub fn new(origin: Point, radius: f32) -> Self {
        Self { origin, radius }
    }

    pub fn on_sphere(&self, point: &Point) -> bool {
        let dis = point.distance(&self.origin);
        if dis.fuzzy_eq(&self.radius) {
            return true;
        }
        false
    }
}

impl FuzzyEq for Sphere {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        self.origin.fuzzy_eq(&other.origin) && self.radius.fuzzy_eq(&other.radius)
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.fuzzy_eq(other)
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];
        let a = ray.direction.dot(&ray.direction);
        let sphere_to_ray = Vector::from_points(&self.origin, &ray.origin);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - self.radius.powi(2);
        let des = b.powi(2) - 4.0 * a * c;
        if des < 0.0 {
            intersections
        } else {
            let t_1 = (-b - des.sqrt()) / (2.0 * a);
            let surface_point_1 = ray.point_at(t_1);
            let normal_v_1 = self.norm_at(&surface_point_1).unwrap();
            intersections.push(Intersection::new(
                t_1,
                ray.direction,
                surface_point_1,
                normal_v_1,
            ));

            let t_2 = (-b + des.sqrt()) / (2.0 * a);
            let surface_point_2 = ray.point_at(t_2);
            let normal_v_2 = self.norm_at(&surface_point_2).unwrap();
            intersections.push(Intersection::new(
                t_2,
                ray.direction,
                surface_point_2,
                normal_v_2,
            ));
            intersections
        }
    }

    fn norm_at(&self, point: &Point) -> Result<Vector, String> {
        if self.on_sphere(point) {
            let v = Vector::from_points(&self.origin, point);
            v.unit()
        } else {
            Err("Point not on the sphere".to_string())
        }
    }
}

#[test]
fn ray_intersect() {
    let sphere = Sphere::new(Point::point(0.0, 0.0, 0.0), 5.0);
    let ray = Ray::new(Point::point(0.0, 0.0, -10.0), Vector::vector(0.0, 0.0, 1.0)).unwrap();
    sphere.intersect(&ray).into_iter().for_each(|inter| {
        let normal = sphere.norm_at(&ray.point_at(inter.get_t())).unwrap();
        println!("{:?}", normal);
    });
}
