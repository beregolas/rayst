use crate::color::Color;
use crate::math::Vec3;

pub mod lambertian;

pub trait Material {
    fn brdf(&self, color_in: Color, light_in: Vec3, light_out: Vec3, normal: Vec3) -> Color;
}
