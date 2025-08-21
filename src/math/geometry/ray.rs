use crate::math::algebra::{common::FuzzyEq, matrix::Matrix, point::Point, vector::Vector};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Result<Self, String> {
        match direction.unit() {
            Err(str) => Err(str),
            Ok(norm) => Ok(Self {
                origin,
                direction: norm,
            }),
        }
    }

    pub fn point_at(&self, t: f32) -> Point {
        self.origin + (self.direction * t)
    }

    pub fn transform(&self, m: Matrix<4>) -> Self {
        Self {
            origin: m * self.origin,
            direction: m * self.direction,
        }
    }
}

impl FuzzyEq for Ray {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        self.direction.fuzzy_eq(&other.direction) && self.origin.fuzzy_eq(&other.origin)
    }
}

impl PartialEq for Ray {
    fn eq(&self, other: &Self) -> bool {
        self.fuzzy_eq(other)
    }
}
