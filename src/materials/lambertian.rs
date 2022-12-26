use crate::color::Color;
use crate::materials::Material;
use crate::math::{Vec3, Vector};

pub struct Lambertian {
    color: Color
}

impl Material for Lambertian {
    fn brdf(&self, color_in: Color, light_in: Vec3, _light_out: Vec3, normal: Vec3) -> Color {
        color_in * light_in.dot(&normal)
    }
}
