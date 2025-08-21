use std::ops::{Add, Div, Mul, Neg, Sub};

use super::common::FuzzyEq;

#[derive(Clone, Copy, Debug)]
pub struct Quaternion {
    r: f32,
    i: f32,
    j: f32,
    k: f32,
}

impl Quaternion {
    pub fn new(r: f32, i: f32, j: f32, k: f32) -> Self {
        Self { r, i, j, k }
    }
    pub fn zero() -> Self {
        Self {
            r: 0.0,
            i: 0.0,
            j: 0.0,
            k: 0.0,
        }
    }

    pub fn get_r(&self) -> f32 {
        self.r
    }
    pub fn get_i(&self) -> f32 {
        self.i
    }
    pub fn get_j(&self) -> f32 {
        self.j
    }
    pub fn get_k(&self) -> f32 {
        self.k
    }
    pub fn get_value(&self) -> (f32, f32, f32, f32) {
        (self.get_r(), self.get_i(), self.get_j(), self.get_k())
    }

    pub fn set_r(&mut self, r: f32) {
        self.r = r;
    }
    pub fn set_i(&mut self, i: f32) {
        self.i = i;
    }
    pub fn set_j(&mut self, j: f32) {
        self.j = j;
    }
    pub fn set_k(&mut self, k: f32) {
        self.k = k;
    }

    pub fn is_zero(&self) -> bool {
        *self == Self::zero()
    }

    pub fn hamilton_multiplied_by(self, other: Self) -> Self {
        let (r_1, i_1, j_1, k_1) = self.get_value();
        let (r_2, i_2, j_2, k_2) = other.get_value();
        let r_new = r_1 * r_2 - i_1 * i_2 - j_1 * j_2 - k_1 * k_2;
        let i_new = r_1 * i_2 + i_1 * r_2 + j_1 * k_2 - k_1 * j_2;
        let j_new = r_1 * j_2 - i_1 * k_2 + j_1 * r_2 + k_1 * i_2;
        let k_new = r_1 * k_2 + i_1 * j_2 - j_1 * i_2 + k_1 * r_2;
        Self::new(r_new, i_new, j_new, k_new)
    }

    pub fn conjugate(&self) -> Self {
        Self::new(self.get_r(), -self.get_i(), -self.get_j(), -self.get_k())
    }

    pub fn norm(&self) -> f32 {
        let (r, i, j, k) = self.get_value();
        (r.powi(2) + i.powi(2) + j.powi(2) + k.powi(2)).sqrt()
    }

    pub fn unit(&self) -> Result<Self, String> {
        *self / self.norm()
    }
}

impl FuzzyEq for Quaternion {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        self.get_r().fuzzy_eq(&other.get_r())
            && self.get_i().fuzzy_eq(&other.get_i())
            && self.get_j().fuzzy_eq(&other.get_j())
            && self.get_k().fuzzy_eq(&other.get_k())
    }
}

impl PartialEq for Quaternion {
    fn eq(&self, other: &Self) -> bool {
        self.fuzzy_eq(other)
    }
}

impl Add for Quaternion {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.get_r() + rhs.get_r(),
            self.get_i() + rhs.get_i(),
            self.get_j() + rhs.get_j(),
            self.get_k() + rhs.get_k(),
        )
    }
}

impl Neg for Quaternion {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.get_r(), -self.get_i(), -self.get_j(), -self.get_k())
    }
}

impl Sub for Quaternion {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<f32> for Quaternion {
    type Output = Self;

    fn mul(self, s: f32) -> Self::Output {
        Self::new(
            self.get_r() * s,
            self.get_i() * s,
            self.get_j() * s,
            self.get_k() * s,
        )
    }
}

impl Div<f32> for Quaternion {
    type Output = Result<Self, String>;

    fn div(self, s: f32) -> Self::Output {
        match s {
            0.0 => Result::Err(String::from("divided by zero")),
            _ => Result::Ok(self * (1.0 / s)),
        }
    }
}
