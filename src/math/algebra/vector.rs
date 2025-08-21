use super::{
    common::{Dimension4, FuzzyEq},
    point::Point,
};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector {
    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn unit_x() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
    pub fn unit_y() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        }
    }
    pub fn unit_z() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 0.0,
        }
    }

    pub fn from_point(point: &Point) -> Self {
        Self::vector(point.get_x(), point.get_y(), point.get_z())
    }

    pub fn from_points(from: &Point, to: &Point) -> Self {
        Vector::vector(
            to.get_x() - from.get_x(),
            to.get_y() - from.get_y(),
            to.get_z() - from.get_z(),
        )
    }

    pub fn get_value(&self) -> (f32, f32, f32) {
        (self.get_x(), self.get_y(), self.get_z())
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

    pub fn is_zero(self) -> bool {
        self == Self::zero()
    }

    pub fn cross(&self, other: &Self) -> Self {
        let (x_1, y_1, z_1) = self.get_value();
        let (x_2, y_2, z_2) = other.get_value();
        let x_new = y_1 * z_2 - z_1 * y_2;
        let y_new = z_1 * x_2 - x_1 * z_2;
        let z_new = x_1 * y_2 - y_1 * x_2;
        Self::vector(x_new, y_new, z_new)
    }

    pub fn dot(&self, other: &Self) -> f32 {
        let (x_1, y_1, z_1) = self.get_value();
        let (x_2, y_2, z_2) = other.get_value();
        x_1 * x_2 + y_1 * y_2 + z_1 * z_2
    }

    pub fn norm(&self) -> f32 {
        self.dot(&self).sqrt()
    }

    pub fn unit(&self) -> Result<Self, String> {
        *self / self.norm()
    }

    pub fn translate_by(&mut self, other: &Self) {
        self.set_x(self.get_x() + other.get_x());
        self.set_y(self.get_y() + other.get_y());
        self.set_z(self.get_z() + other.get_z());
    }

    /// in radius
    pub fn angle_with(&self, other: &Self) -> Option<f32> {
        let (unit_1, unit_2) = (self.unit(), other.unit());
        match (unit_1, unit_2) {
            (Ok(val_1), Ok(val_2)) => {
                let product = val_1.dot(&val_2);
                if product.fuzzy_eq(&1.0) {
                    Some(1.0_f32.acos())
                } else if product.fuzzy_eq(&-1.0) {
                    Some(-1.0_f32.acos())
                } else {
                    Some(product.acos())
                }
            }
            _ => None,
        }
    }

    pub fn reflect(&self, normal: &Self) -> Result<Self, String> {
        if normal.norm().fuzzy_eq(&0.0) {
            Err("Norm has 0 magnitude".to_string())
        } else {
            Ok(-*self + (*normal) * 2.0 * self.dot(normal))
        }
    }
}

impl Dimension4 for Vector {
    type Value = f32;
    fn new(x: Self::Value, y: Self::Value, z: Self::Value, w: Self::Value) -> Self {
        Vector { x, y, z, w }
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

impl FuzzyEq for Vector {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        self.get_x().fuzzy_eq(&other.get_x())
            && self.get_y().fuzzy_eq(&other.get_y())
            && self.get_z().fuzzy_eq(&other.get_z())
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.fuzzy_eq(other)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::vector(
            self.get_x() + rhs.get_x(),
            self.get_y() + rhs.get_y(),
            self.get_z() + rhs.get_z(),
        )
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::vector(-self.get_x(), -self.get_y(), -self.get_z())
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, s: f32) -> Self::Output {
        Self::vector(self.get_x() * s, self.get_y() * s, self.get_z() * s)
    }
}

impl Div<f32> for Vector {
    type Output = Result<Self, String>;

    fn div(self, s: f32) -> Self::Output {
        match s {
            0.0 => Result::Err(String::from("divided by zero")),
            _ => Result::Ok(self * (1.0 / s)),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn vector_work() {}
}
