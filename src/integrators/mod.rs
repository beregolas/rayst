use crate::color::Color;
use crate::ray::Ray;
use crate::world::World;

pub mod ray_trace;

pub trait Integrator {
    fn li(&self, ray: &Ray) -> Color;
}