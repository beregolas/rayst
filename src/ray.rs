use crate::math::{EPSILON, Vec3, Vector};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub recip_direction: Vec3,  // 1. / direction
    pub min_distance: f32,
    pub max_distance: f32
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, min_distance: Option<f32>, max_distance: Option<f32>) -> Self {
        // line automatically disabled in release builds
        // supplying a vector of length 0 at runtime will result in (NAN, NAN, NAN) as a vector
        debug_assert_ne!(direction.length_squared(), 0.);
        let d: Vec3 = direction.normalized();
        Ray {
            origin,
            direction: d,
            recip_direction: 1. / d,
            min_distance: min_distance.unwrap_or(EPSILON),
            max_distance: max_distance.unwrap_or(f32::INFINITY),
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
