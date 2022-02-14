mod aabb;

use bevy_math::Vec3;
use crate::geometry::aabb::Aabb;
use crate::ray::Ray;

pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32
}

impl Hit {
    pub fn new(point: Vec3, normal: Vec3, distance: f32) -> Self {
        Hit {
            point,
            normal: normal.normalize(),
            distance
        }
    }
}

pub trait Geometry {
    fn intersect(&self, ray: Ray) -> Option<Hit>;
    fn does_intersect(&self, ray: Ray) -> bool;
    fn bounds(&self) -> Aabb;
}