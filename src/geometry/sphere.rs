use crate::geometry::{Geometry, Hit};
use crate::math::{Vec3, Vector};
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius2: f32
}

impl Sphere {
    pub fn new(c: Vec3, r: f32) -> Self {
        Sphere {
            center: c,
            radius2: r * r
        }
    }
}

impl Geometry for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        // we don't accept inner hits
        // vector from the ray origin to the center of the sphere
        let d = ray.origin - self.center;
        // the distance along the ray
        let distance = -d.dot(&ray.direction);
        // projected onto the ray direction. This is the closest point to the center on the ray
        let point = ray.at(distance);
        // height of the closest point over the center of the sphere
        let height2 = (self.center - point).length_squared();
        // nearest point outside of sphere: no hit
        if height2 > self.radius2 {
            return None;
        }
        // find the most backwards hit point on the ray. if it's behind the origin, disregard (inside or behind the sphere)
        let hit_distance = distance - (self.radius2 - height2).sqrt();
        if hit_distance < 0. {
            return None;
        }
        let hit_point = ray.at(hit_distance);
        Some(Hit{
            point: hit_point,
            normal: (hit_point - self.center).normalized(),
            distance: hit_distance
        })
    }

    fn does_intersect(&self, ray: &Ray) -> bool {
        // we don't accept inner hits
        // vector from the ray origin to the center of the sphere
        let d = ray.origin - self.center;
        // the distance along the ray
        let distance = -d.dot(&ray.direction);
        // projected onto the ray direction. This is the closest point to the center on the ray
        let point = ray.at(distance);
        // height of the closest point over the center of the sphere
        let height2 = (self.center - point).length_squared();
        // nearest point outside of sphere: no hit
        if height2 > self.radius2 {
            return false;
        }
        let t = (self.radius2 - height2).sqrt();
        distance > t.abs()
    }

    fn get_bounds(&self) -> f32 {
        todo!()
    }
}

#[cfg(test)]
mod vec3_tests {
    use crate::geometry::{Geometry, Sphere};
    use crate::math::Vec3;
    use crate::ray::Ray;

    #[test]
    fn does_intersect1() {
        let s = Sphere::new(Vec3::new(0., 0., 0.), 1.0);
        let ray1 = Ray::new(Vec3::new(-2., 0., 0.), Vec3::new(1., 0.5, 0.));
        assert!(s.does_intersect(&ray1));
    }

}