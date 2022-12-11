use crate::math::{Vec3, Vector};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub recip_direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        // line automatically disabled in release builds
        // supplying a vector of length 0 at runtime will result in (NAN, NAN, NAN) as a vector
        debug_assert_ne!(direction.length_squared(), 0.);
        let d: Vec3 = direction.normalized();
        Ray {
            origin,
            direction: d,
            recip_direction: 1. / d,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
