use crate::geometry::{Geometry, Hit};
use crate::math::{Vec3, Vector};
use crate::ray::Ray;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Aabb {
            min: min.min_vector(&max),
            max: max.max_vector(&min)
        }
    }

    pub fn inner_does_intersect(&self, ray: &Ray) -> bool {
        // efficient slab algorithm
        let t0 = (self.min - ray.origin) * ray.recip_direction;
        let t1 = (self.max - ray.origin) * ray.recip_direction;
        let t_min = t0.min_vector(&t1);
        let t_max = t0.max_vector(&t1);
        let potential_hit_dist = t_max.min_component();
        // only accept hits, when we are inside the box or in front of it
        t_min.max_component() <= potential_hit_dist && potential_hit_dist >= 0.
    }

}

impl Geometry for Aabb {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        // the same thing as the efficient slab algorithm, just with more information and slower steps
        let t0 = (self.min - ray.origin) * ray.recip_direction;
        let t1 = (self.max - ray.origin) * ray.recip_direction;
        let t_min = t0.min_vector(&t1);
        let t_max = t0.max_vector(&t1);
        let potential_hit_dist = t_min.max_component();
        if !(potential_hit_dist <= t_max.min_component() && potential_hit_dist >= 0.) {
            return None
        }
        // find the last axis we cross in t_min
        let axis = if t_min.x > t_min.y {
            if t_min.x > t_min.z {
                0
            } else {
                2
            }
        } else if t_min.y > t_min.z {
            1
        } else {
            2
        };
        Some(
            Hit {
                distance: potential_hit_dist,
                point: ray.at(potential_hit_dist),
                normal: Vec3::AXES[axis] * -ray.direction.dot(&Vec3::AXES[axis]).signum()
            }
        )
    }

    fn does_intersect(&self, ray: &Ray) -> bool {
        // efficient slab algorithm
        let t0 = (self.min - ray.origin) * ray.recip_direction;
        let t1 = (self.max - ray.origin) * ray.recip_direction;
        let t_min = t0.min_vector(&t1);
        let t_max = t0.max_vector(&t1);
        let potential_hit_dist = t_min.max_component();
        // only accept hits if we hit the box before us from the outside (inside and behind us is ignored)
        potential_hit_dist <= t_max.min_component() && potential_hit_dist >= 0.
    }

    fn get_bounds(&self) -> Aabb {
        *self
    }
}

#[cfg(test)]
mod aabb_tests {
    use crate::geometry::aabb::Aabb;
    use crate::geometry::Geometry;
    use crate::math::Vec3;
    use crate::ray::Ray;

    #[test]
    fn does_intersect1() {
        let box1 = Aabb::new(Vec3::new(-1., -1., -1.), Vec3::new(1., 1., 1.));
        let r1 = Ray::new(Vec3::new(-1.001, 0., 0.), Vec3::new(1., 10., 110.));
        assert!(box1.inner_does_intersect(&r1));

    }

    #[test]
    fn get_bounds() {
        let mut box1 = Aabb::new(Vec3::new(-1., -1., -1.), Vec3::new(1., 1., 1.));
        let box2 = box1.get_bounds();
        box1.min.x = -5.;
        assert_eq!(box1.min.x, -5.);
        assert_eq!(box2.min.x, -1.);
    }

}