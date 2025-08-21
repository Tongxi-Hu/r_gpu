use crate::math::{
    algebra::{common::FuzzyEq, vector::Vector},
    geometry::common::Intersection,
};

use super::{color::Color, common::Illuminated, light::PointLight};

pub struct Phong {
    pub color: Color,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
    reflectiveness: f32,
}

impl Default for Phong {
    fn default() -> Self {
        Phong {
            color: Color::rgb(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflectiveness: 0.0,
        }
    }
}

impl Phong {
    pub fn metal() -> Self {
        Phong {
            color: Color::rgb(0.0, 0.0, 0.0),
            ambient: 0.0,
            diffuse: 0.4,
            specular: 0.9,
            shininess: 200.0,
            reflectiveness: 1.0,
        }
    }

    pub fn with_color(mut self, color: &Color) -> Self {
        self.color = *color;
        self
    }

    pub fn with_ambient(mut self, ambient: f32) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn with_diffuse(mut self, diffuse: f32) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_specular(mut self, specular: f32) -> Self {
        self.specular = specular;
        self
    }

    pub fn with_shininess(mut self, shininess: f32) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn with_reflectiveness(mut self, reflectiveness: f32) -> Self {
        self.reflectiveness = reflectiveness;
        self
    }
}

impl FuzzyEq for Phong {
    fn fuzzy_eq(&self, other: &Phong) -> bool {
        self.color.fuzzy_eq(&other.color)
            && self.ambient.fuzzy_eq(&other.ambient)
            && self.diffuse.fuzzy_eq(&other.diffuse)
            && self.specular.fuzzy_eq(&other.specular)
            && self.shininess.fuzzy_eq(&other.shininess)
    }
}

impl Illuminated for Phong {
    fn lighting(&self, light: &PointLight, intersection: &Intersection) -> Color {
        let effective_color = self.color * light.intensity;
        let ambient_color = effective_color * self.ambient;
        let light_v_try =
            Vector::from_points(&intersection.get_surface_point(), &light.position).unit();
        match light_v_try {
            Err(_) => Color::black() + Color::black() + Color::black(),
            Ok(light_v) => {
                let light_normal = light_v.dot(&intersection.get_normal());
                if light_normal < 0.0 {
                    //light on the other side of the surface
                    return ambient_color + Color::black() + Color::black();
                } else {
                    //light on the same side of the surface
                    let diffuse_color = effective_color * self.diffuse * light_normal;
                    let reflect_v_try = light_v.reflect(&intersection.get_normal());
                    match reflect_v_try {
                        Err(_) => ambient_color + diffuse_color + Color::black(),
                        Ok(reflect_v) => {
                            let reflect_eye = reflect_v.dot(&intersection.get_eye_v());
                            if reflect_eye <= 0.0 {
                                return ambient_color + diffuse_color + Color::black();
                            } else {
                                return ambient_color
                                    + diffuse_color
                                    + light.intensity
                                        * self.specular
                                        * reflect_eye.powf(self.shininess);
                            }
                        }
                    }
                }
            }
        }
    }

    fn reflective(&self) -> bool {
        self.reflectiveness.fuzzy_eq(&1.0)
    }

    fn reflect_light(&self, color: &Color) -> Color {
        *color * self.reflectiveness
    }
}
