use crate::color::Color;
use crate::geometry::Geometry;
use crate::lights::LightSource;
use crate::math::{Vec3, Vector};
use crate::ray::Ray;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PointLight {
    pub center: Vec3,
    pub intensity: Color
}

impl PointLight {
    pub fn new(center: Vec3, intensity: Color) -> Self {
        PointLight {
            center, intensity
        }
    }
}

impl LightSource for PointLight {
    fn sample(&self, point: Vec3, world: &dyn Geometry) -> (Color, Vec3) {
        let direction = self.center - point;
        // TODO: Optimize usage of length
        let hit = world.does_intersect(&Ray::new(point, direction, None, Some(direction.length())));
        if !hit {
            (self.intensity / direction.length_squared(), direction.normalized())
        } else {
            (Color::new(0., 0., 0.), direction.normalized())
        }
    }
}
