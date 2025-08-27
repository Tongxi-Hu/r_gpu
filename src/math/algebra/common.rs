use core::f32;

use crate::constant::EPSILON;

pub trait FuzzyEq {
    fn fuzzy_eq(&self, other: &Self) -> bool;
}

impl FuzzyEq for f32 {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        (*self - *other).abs() <= EPSILON
    }
}

pub fn deg_to_rad(deg: f32) -> f32 {
    f32::consts::PI * (deg / 180.0)
}

pub trait Dimension4 {
    type Value;
    fn new(x: Self::Value, y: Self::Value, z: Self::Value, w: Self::Value) -> Self;
    fn get_x(&self) -> Self::Value;
    fn get_y(&self) -> Self::Value;
    fn get_z(&self) -> Self::Value;
    fn get_w(&self) -> Self::Value;
    fn get_raw(&self) -> [Self::Value; 4];
}

pub trait Determinant {
    fn det(&self) -> f32;
}
