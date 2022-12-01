use crate::geometry::{Geometry, Hit};
use crate::math::Vec3;
use crate::ray::Ray;

struct Sphere {
    center: Vec3,
    radius: f32
}

impl Geometry for Sphere {
    fn intersect(&self, ray: &Ray) -> Hit {
        todo!()
    }

    fn does_intersect(&self, ray: &Ray) -> bool {
        todo!()
    }

    fn get_bounds(&self) -> f32 {
        todo!()
    }
}