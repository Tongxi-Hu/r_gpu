use std::ops::{Add, Neg};

use super::{
    common::{Dimension4, FuzzyEq},
    vector::Vector,
};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

unsafe impl bytemuck::Zeroable for Point {}

unsafe impl bytemuck::Pod for Point {}

impl Point {
    pub fn origin() -> Self {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn is_origin(&self) -> bool {
        *self == Self::origin()
    }

    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    pub fn get_value(&self) -> (f32, f32, f32) {
        (self.get_x(), self.get_y(), self.get_z())
    }

    pub fn distance(&self, other: &Self) -> f32 {
        ((self.get_x() - other.get_x()).powi(2)
            + (self.get_y() - other.get_y()).powi(2)
            + (self.get_z() - other.get_z()).powi(2))
        .sqrt()
    }

    pub fn distance_from_origin(&self) -> f32 {
        let origin = Self::origin();
        self.distance(&origin)
    }

    pub fn to_vector(&self) -> Vector {
        Vector::vector(self.get_x(), self.get_y(), self.get_z())
    }

    pub fn translate(&mut self, amount: &Vector) {
        let (trans_x, trans_y, trans_z) = amount.get_value();
        self.set_x(self.get_x() + trans_x);
        self.set_y(self.get_y() + trans_y);
        self.set_z(self.get_z() + trans_z);
    }
}

impl Dimension4 for Point {
    type Value = f32;
    fn new(x: Self::Value, y: Self::Value, z: Self::Value, w: Self::Value) -> Self {
        Self { x, y, z, w }
    }

    fn get_x(&self) -> Self::Value {
        self.x
    }
    fn get_y(&self) -> Self::Value {
        self.y
    }
    fn get_z(&self) -> Self::Value {
        self.z
    }
    fn get_w(&self) -> Self::Value {
        self.w
    }
}

impl FuzzyEq for Point {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        self.get_x().fuzzy_eq(&other.get_x())
            && self.get_y().fuzzy_eq(&other.get_y())
            && self.get_z().fuzzy_eq(&other.get_z())
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.fuzzy_eq(other)
    }
}

impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::point(-self.get_x(), -self.get_y(), -self.get_z())
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Self::Output {
        Self::point(
            self.get_x() + rhs.get_x(),
            self.get_y() + rhs.get_y(),
            self.get_z() + rhs.get_z(),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::math::algebra::point::Point;

    #[test]
    fn point_work() {
        let mut point_1 = Point::point(1.0, 1.0, 1.0);
        point_1.set_x(4.0);
        let point_2 = Point::point(2.0, 2.0, 2.0);
        println!("{:?}", point_1.distance(&point_2));
    }
}
