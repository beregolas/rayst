use crate::geometry::{Aabb, Geometry, Hit};
use crate::math::{EPSILON, Vec3, Vector};
use crate::ray::Ray;

pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Triangle {
        Triangle {
            v0,
            v1,
            v2
        }
    }
}

impl Geometry for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        // edges 1 and 2 (v0 -> v1 & v1 -> v2)
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;
        // perpendicular vector to the ray and the first edge
        let h = ray.direction.cross(&e2);
        // project the perpendicular vector onto the second edge
        let a = e1.dot(&h);
        // if that projection is too small, the ray is parallel to the triangle
        if a.abs() < EPSILON {
            return None;
        }
        // prepare the inverse of the projection length for computations
        let f = 1. / a;
        // vector from the ray origin to the triangle origin
        let s = ray.origin - self.v0;
        let u = f * s.dot(&h);
        if !(0. ..=1.0).contains(&u) {
            return None;
        }
        let q = s.cross(&e1);
        let v = f * ray.direction.dot(&q);
        if v < 0. || u + v > 1. {
            return None;
        }
        let t = f * e2.dot(&q);
        if t > ray.min_distance && t < ray.max_distance {
            Some(
                Hit {
                    point: ray.at(t),
                    distance: t,
                    normal: e2.cross(&e1).normalized()
                }
            )
        } else {
            None
        }
    }

    fn does_intersect(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }

    fn get_bounds(&self) -> Aabb {
        Aabb {
            min: self.v0.min_vector(&self.v1.min_vector(&self.v2)),
            max: self.v0.max_vector(&self.v1.max_vector(&self.v2))
        }
    }
}



#[cfg(test)]
mod vec3_tests {
    use crate::geometry::Geometry;
    use crate::geometry::triangle::Triangle;
    use crate::math::Vec3;
    use crate::ray::Ray;

    #[test]
    fn intersect() {
        let t1 = Triangle::new(Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.));
        let r1 = Ray::new(Vec3::new(0.1, 0.1, -0.1), Vec3::new(0., 0., 1.), None, None);
        println!("{:?}", t1.intersect(&r1));
        // assert!(t1.intersect(&r1).is_some());
    }

}