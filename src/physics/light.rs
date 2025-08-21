use crate::math::algebra::{common::FuzzyEq, point::Point};

use super::color::Color;
#[derive(Debug)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

impl Default for PointLight {
    fn default() -> Self {
        Self::new(Point::origin(), Color::white())
    }
}

impl FuzzyEq for PointLight {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        self.position.fuzzy_eq(&(other.position)) && self.intensity.fuzzy_eq(&(other.intensity))
    }
}

impl PartialEq for PointLight {
    fn eq(&self, other: &Self) -> bool {
        self.fuzzy_eq(other)
    }
}
