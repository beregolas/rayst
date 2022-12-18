use crate::geometry::{Geometry, Hit};
use crate::math::{Vec3, Vector};
use crate::ray::Ray;

pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Aabb {
            min, max
        }
    }

    pub fn outer_does_intersect(&self, ray: &Ray) -> bool {
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
        todo!()
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

    fn get_bounds(&self) -> f32 {
        todo!()
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
        let r1 = Ray::new(Vec3::new(-1.1, 0., 0.), Vec3::new(1., 0., 0.));
        assert!(box1.does_intersect(&r1));

    }
}