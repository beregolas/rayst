use crate::color::Color;
use crate::geometry::Geometry;
use crate::math::Vec3;

pub mod point;

pub trait LightSource {
    fn sample(point: Vec3, world: &dyn Geometry) -> Color;
}
