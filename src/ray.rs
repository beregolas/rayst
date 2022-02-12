use bevy_math::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {

    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        // line automatically disabled in release builds
        // supplying a vector of length 0 at runtime will result in (NAN, NAN, NAN) as a vector
        debug_assert_ne!(direction.length_squared(), 0.);
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
        let _ = Ray::new(
            Vec3::new(10., 10., 10.),
            Vec3::ZERO
        );
    }

    #[test]
    fn at_zero() {
        let r1 = Ray::new(
            Vec3::new(0., 0., 0.),
            Vec3::new(-10., 0., 0.)
        );
        assert_eq!(r1.at(666.666), Vec3::new(-666.666, 0., 0.));
        assert_eq!(r1.at(-420.69), Vec3::new(420.69, 0., 0.));
        assert_eq!(r1.at(0.), Vec3::ZERO);

        let r2 = Ray::new(
            Vec3::new(0., 0., 0.),
            Vec3::new(5., -7., 199.)
        );
        assert_eq!(r2.at(88_000.), Vec3::new(5., -7., 199.).normalize() * 88_000.);
        assert_eq!(r2.at(-9.), Vec3::new(5., -7., 199.).normalize() * -9.);
        assert_eq!(r2.at(0.), Vec3::ZERO);
    }

    #[test]
    fn at() {
        let r1 = Ray::new(
            Vec3::new(77.5, -0.004, 1.7),
            Vec3::new(-10.5, 0., 0.1)
        );
        assert_eq!(r1.at(1.), Vec3::new(77.5, -0.004, 1.7) + Vec3::new(-10.5, 0., 0.1).normalize());
        assert_eq!(r1.at(-1.), Vec3::new(77.5, -0.004, 1.7) - Vec3::new(-10.5, 0., 0.1).normalize());
        assert_eq!(r1.at(0.5), Vec3::new(77.5, -0.004, 1.7) + Vec3::new(-10.5, 0., 0.1).normalize() / 2.);
    }

}