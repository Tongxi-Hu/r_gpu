use crate::math::geometry::common::Intersection;

use super::{color::Color, light::PointLight};

pub trait Illuminated {
    ///all vector start from position in the calculation  
    fn lighting(
        &self,
        //incoming light source
        light: &PointLight,
        intersection: &Intersection,
    ) -> Color;

    fn reflective(&self) -> bool;

    fn reflect_light(&self, color: &Color) -> Color;
}
