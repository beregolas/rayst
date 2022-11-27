use bevy_math::Vec3;
use crate::geometry::{Geometry, Hit};
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Aabb{
    pub min: Vec3,
    pub max: Vec3
}

impl Geometry for Aabb {
    fn intersect(&self, ray: Ray) -> Option<Hit> {
        // intersection test with hit points
        // holds farthest intersect with closest planes of each axis
        let mut t_min: f32 = f32::NEG_INFINITY;
        // holds closest intersect with farthest planes of each axis
        let mut t_max: f32 = f32::INFINITY;

        let mut norm: usize = 0;
        let mut sign: i32 = 1;

        // check all 2 planes in all 3 axis
        for i in 0..3 {
            // compute hit-point on axis plane for min and max in dimension i
            let t1: f32 = (self.min[i] - ray.origin[i]) * ray.recip_direction[i];
            let t2: f32 = (self.max[i] - ray.origin[i]) * ray.recip_direction[i];

            // update the farthest closest and closest farthest hit-points
            if t1 < t2 {
                if t2 < t_max {
                    t_max = t2;
                }
                if t1 > t_min {
                    t_min = t1;
                    norm = i;
                    sign = 1;
                }
            } else {
                if t1 < t_max {
                    t_max = t1;
                }
                if t2 > t_min {
                    t_min = t2;
                    norm = i;
                    sign = -1;
                }
            }
        };
        // if t_max is farther away than t_min and the hit-point is > 0 (in front of the ray), we hit
        if !(t_max > t_min.max(0.)) {
            return None;
        };
        // we have a hit at t_min, compute hit point
        let hit_point: Vec3 = ray.at(t_min);
        let normal: Vec3 = Vec3::AXES[norm];
        Some(Hit::new(hit_point, normal, t_min))
    }

    fn does_intersect(&self, ray: Ray) -> bool {
        // branch-less, fast intersection test
        // holds farthest intersect with closest planes of each axis
        let mut t_min = f32::NEG_INFINITY;
        // holds closest intersect with farthest planes of each axis
        let mut t_max = f32::INFINITY;
        // check all 2 planes in all 3 axis
        for i in 0..3 {
            // compute hit-point on axis plane for min and max in dimension i
            let t1 = (self.min[i] - ray.origin[i]) * ray.recip_direction[i];
            let t2 = (self.max[i] - ray.origin[i]) * ray.recip_direction[i];

            // update the farthest closest and closest farthest hit-points
            t_min = t_min.max(t1.min(t2));
            t_max = t_max.min(t1.max(t2));
        };
        // if t_max is farther away than t_min and the hit-point is > 0 (in front of the ray), we hit
        t_max > t_min.max(0.)
    }

    fn bounds(&self) -> Aabb {
        self.clone()
    }
}

#[cfg(test)]
mod aabb_tests {
    
}