use bevy_math::Vec3;
use crate::geometry::{Geometry, Hit};
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Aabb{
    pub min: Vec3,
    pub max: Vec3
}

impl Geometry for Aabb {
    fn intersect(&self, r: Ray) -> Option<Hit> {
        todo!()
    }

    fn does_intersect(&self, r: Ray) -> bool {
        todo!()
    }

    fn bounds(&self) -> Aabb {
        self.clone()
    }
}