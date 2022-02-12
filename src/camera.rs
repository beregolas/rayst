use bevy_math::{Vec2, Vec3};
use crate::ray::Ray;


pub trait Camera {
    // input: coords from (0,0) to (1,1)
    fn at(&self, coords: Vec2) -> Ray;
}


fn prepare_input(coords: Vec2) -> Vec2 {
    (coords - 0.5) * 2
}


#[derive(Debug)]
pub struct OrthographicCamera {
    // Forward is normalized
    // up and right are scaled to half width & height
    origin: Vec3,
    forward: Vec3,
    up: Vec3,
    right: Vec3
}

impl OrthographicCamera {
    pub fn new(origin: Vec3, forward: Vec3, up: Vec3, size: Vec2) -> Self {
        let f = forward.normalize();
        let u = up.normalize();
        debug_assert!(!forward.is_nan());
        debug_assert!(!up.is_nan());
        let r = f.cross(u).normalize();
        Self {
            origin,
            forward: f,
            right: r * size.x,
            up: r.cross(f).normalize() * size.y
        }
    }
}

impl Camera for OrthographicCamera {
    fn at(&self, coords: Vec2) -> Ray {
        // take input from 0 to 1 and normalize it to -1 to 1
        let c = prepare_input(coords);
        Ray{
            origin: self.up * -c.y + self.right * c.x,
            direction: self.forward
        }
    }
}

#[derive(Debug)]
pub struct PerspectiveCamera {
    // Forward is scaled to the distance to the camera plane
    // up and right are scaled to half width & height
    origin: Vec3,
    forward: Vec3,
    up: Vec3,
    right: Vec3
}

impl PerspectiveCamera {
    pub fn new(origin: Vec3, forward: Vec3, up: Vec3, size: Vec2, fov: f32) -> Self {
        return Self {
            origin,

        }
    }
}

impl Camera for PerspectiveCamera {
    fn at(&self, coords: Vec2) -> Ray {
        todo!()
    }
}