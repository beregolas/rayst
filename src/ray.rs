use bevy_math::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {

    pub fn new(origin: Vec3, direction: Vec3) -> Self{
        Ray{
            origin,
            direction: direction.normalize()
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

}


#[cfg(test)]
mod ray_tests {
    use bevy_math::Vec3;
    use crate::ray::Ray;

    #[test]
    fn create_success() {
        let r1 = Ray::new(
            Vec3::new(0., 0., 0.),
            Vec3::new(1., 0., 0.)
        );
        assert_eq!(r1.origin, Vec3::new(0., 0., 0.));
        assert_eq!(r1.direction, Vec3::new(1., 0., 0.));
        let r2 = Ray::new(
            Vec3::new(10., 10., 10.),
            Vec3::new(10., 0., 0.)
        );
        assert_eq!(r2.origin, Vec3::new(10., 10., 10.));
        assert_eq!(r2.direction, Vec3::new(1., 0., 0.));
        let r3 = Ray::new(
            Vec3::new(-10., 6.3, -999999.999999),
            Vec3::new(-9., 9., 27.)
        );
        assert_eq!(r3.origin, Vec3::new(-10., 6.3, -999999.999999));
        assert_eq!(r3.direction, Vec3::new(-1., 1., 3.).normalize());
    }

    #[test]
    #[should_panic]
    fn create_failure() {
        let r = Ray::new(
            Vec3::new(10., 10., 10.),
            Vec3::ZERO
        );
        println!("{:?}", r);
    }

}