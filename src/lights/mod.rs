use crate::color::Color;
use crate::math::Vec3;
use crate::world::World;

pub mod point;

pub trait LightSource {
    fn sample(&self, point: Vec3, world: &World) -> (Color, Vec3);
}
