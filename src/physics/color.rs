use std::ops::{Add, Mul};

use crate::math::algebra::common::FuzzyEq;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
    pub fn black() -> Self {
        Color::rgb(0.0, 0.0, 0.0)
    }
    pub fn white() -> Self {
        Color::rgb(1.0, 1.0, 1.0)
    }

    pub fn set_r(&mut self, r: f32) {
        self.r = r;
    }
    pub fn set_g(&mut self, g: f32) {
        self.g = g;
    }
    pub fn set_b(&mut self, b: f32) {
        self.b = b;
    }

    pub fn get_r(&self) -> f32 {
        self.r
    }
    pub fn get_g(&self) -> f32 {
        self.g
    }
    pub fn get_b(&self) -> f32 {
        self.b
    }

    pub fn get_value(&self) -> (f32, f32, f32) {
        (self.get_r(), self.get_g(), self.get_b())
    }

    pub fn clamp(&self, low: f32, high: f32) -> Self {
        Self::rgb(
            self.r.max(low).min(high),
            self.g.max(low).min(high),
            self.b.max(low).min(high),
        )
    }
}

impl FuzzyEq for Color {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        self.get_r().fuzzy_eq(&other.get_r())
            && self.get_g().fuzzy_eq(&other.get_g())
            && self.get_b().fuzzy_eq(&other.get_b())
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.fuzzy_eq(other)
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color::rgb(
            self.get_r() + rhs.get_r(),
            self.get_g() + rhs.get_g(),
            self.get_b() + rhs.get_b(),
        )
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Self::Output {
        Color::rgb(
            self.get_r() * other.get_r(),
            self.get_g() * other.get_g(),
            self.get_b() * other.get_b(),
        )
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, other: f32) -> Self::Output {
        Color::rgb(
            self.get_r() * other,
            self.get_g() * other,
            self.get_b() * other,
        )
    }
}
