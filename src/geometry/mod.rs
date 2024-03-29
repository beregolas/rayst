mod sphere;
mod aabb;
mod triangle;

use crate::math::Vec3;
use crate::ray::Ray;

pub use sphere::Sphere;
pub use aabb::Aabb;
pub use triangle::Triangle;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
}

pub trait Geometry {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
    fn does_intersect(&self, ray: &Ray) -> bool;
    fn get_bounds(&self) -> Aabb;
}