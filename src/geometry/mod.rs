mod sphere;

use crate::math::Vec3;
use crate::ray::Ray;

pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Geometry {
    fn intersect(&self, ray: &Ray) -> Hit;
    fn does_intersect(&self, ray: &Ray) -> bool;
    fn get_bounds(&self) -> f32;
}