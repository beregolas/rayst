use std::mem::swap;
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
        let l = self.center - ray.origin;
        let tca = l.dot(&ray.direction);
        let d2 = l.dot(&l) - tca * tca;
        if d2 > self.radius * self.radius {
            return false;
        }
        let thc = (self.radius - d2).sqrt();
        let mut t0 = tca - thc;
        let mut t1 = tca + thc;

        if t0 > t1 {
            swap(&mut t0, &mut t1)
        }

        if t0 < 0. {
            t0 = t1;
            if t0 < 0. {
                return false;
            }
        }


        true
    }

    fn get_bounds(&self) -> f32 {
        todo!()
    }
}