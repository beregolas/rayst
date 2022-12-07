use crate::geometry::{Geometry, Hit};
use crate::math::Vec3;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

impl Geometry for Sphere {
    fn intersect(&self, ray: &Ray) -> Hit {
        todo!()
    }

    fn does_intersect(&self, ray: &Ray) -> bool {
        // vector from ray origin to the center of the sphere
        let d = ray.origin - self.center;
        // projected onto the ray direction. This is the closest point to the center on the ray
        let P = ray.at(d.dot(&ray.direction));
        (self.center - P).length_squared() < self.radius * self.radius
    }

    fn get_bounds(&self) -> f32 {
        todo!()
    }
}