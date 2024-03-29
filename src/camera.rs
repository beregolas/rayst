use crate::ray::Ray;
use crate::math::{Vec2, Vec3, Vector};

pub trait Camera {
    // input: coords from (0,0) to (1,1)
    fn at(&self, coords: Vec2) -> Ray;
}

fn prepare_input(coords: Vec2) -> Vec2 {
    (coords - 0.5) * 2.
}

#[derive(Debug)]
pub struct OrthographicCamera {
    // Forward is normalized
    // up and right are scaled to half width & height
    origin: Vec3,
    forward: Vec3,
    up: Vec3,
    right: Vec3,
}

impl OrthographicCamera {
    pub fn new(origin: Vec3, forward: Vec3, up: Vec3, size: Vec2) -> Self {
        let f = forward.normalized();
        let u = up.normalized();
        debug_assert!(!f.is_nan());
        debug_assert!(!u.is_nan());
        let r = f.cross(&u).normalized();
        Self {
            origin,
            forward: f,
            right: r * size.x,
            up: r.cross(&f).normalized() * size.y,
        }
    }
}

impl Camera for OrthographicCamera {
    fn at(&self, coords: Vec2) -> Ray {
        let c = prepare_input(coords);
        Ray::new(self.origin + self.up * -c.y + self.right * c.x, self.forward, None, None)
    }
}

#[derive(Debug)]
pub struct PerspectiveCamera {
    // Forward is normalized
    // up and right are scaled to half width & height of image plane at distance 1
    origin: Vec3,
    forward: Vec3,
    up: Vec3,
    right: Vec3,
}

impl PerspectiveCamera {
    pub fn new(origin: Vec3, forward: Vec3, up: Vec3, aspect: f32, fov: f32) -> Self {
        let f = forward.normalized();
        let u = up.normalized();
        debug_assert!(!f.is_nan());
        debug_assert!(!u.is_nan());
        let r = f.cross(&u).normalized();
        // opening angle between forward and view border
        let alpha = fov / 180. * std::f32::consts::PI / 2.;
        let size = Vec2::new(alpha.tan(), alpha.tan() / aspect);
        Self {
            origin,
            forward: f,
            right: r * size.x,
            up: u * size.y,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn at(&self, coords: Vec2) -> Ray {
        let c = prepare_input(coords);
        Ray::new(
            self.origin,
            self.forward + self.up * -c.y + self.right * c.x,
            None,
            None
        )
    }
}

#[cfg(test)]
mod camera_tests {
    use crate::camera::OrthographicCamera;
    use crate::math::{ApproxEq, Vec2, Vec3};

    #[test]
    fn create_orthographic() {
        let cam1 = OrthographicCamera::new(Vec3::new(1., 1., 1.), Vec3::new(-4., 0., 0.), Vec3::new(0., 90.3,  0.), Vec2::new(15., 15.));
        assert!(cam1.forward.a_eq(&Vec3::new(-1., 0., 0.)));
        assert!(cam1.origin.a_eq(&Vec3::new(1., 1., 1.)));
        assert!(cam1.up.a_eq(&Vec3::new(0., 15., 0.)));
        assert!(cam1.right.a_eq(&Vec3::new(0., 0., -15.)))
    }

}