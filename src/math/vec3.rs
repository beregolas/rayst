use std::ops::{Index, IndexMut};
use crate::{vec_op};
use crate::math::vec::Vector;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    pub const ONE: Self = Vec3 {
        x: 1.,
        y: 1.,
        z: 1.,
    };

    pub const UP: Self = Vec3 {
        x: 0.,
        y: 0.,
        z: 1.,
    };

    pub const FORWARD: Self = Vec3 {
        x: 1.,
        y: 0.,
        z: 0.,
    };

    pub const RIGHT: Self = Vec3 {
        x: 0.,
        y: 1.,
        z: 0.,
    };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Vector for Vec3 {
    fn normalize(&mut self) {
        let length: f32 = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    fn normalized(&self) -> Self {
        let mut new_vector: Vec3 = *self;
        new_vector.normalize();
        new_vector
    }

    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn is_nan(&self) -> bool {
        self.x.is_nan() && self.y.is_nan() && self.z.is_nan()
    }

    fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

vec_op!(Vec3, +, x y z);
vec_op!(Vec3, -, x y z);
vec_op!(Vec3, *, x y z);
vec_op!(Vec3, /, x y z);

impl From<f32> for Vec3 {
    fn from(value: f32) -> Self {
        Vec3 {
            x: value,
            y: value,
            z: value,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of bounds"),
        }
    }
}


#[cfg(test)]
mod vec3_tests {
    use crate::math::{EPSILON, Vec3};

    #[test]
    fn creation1() {
        let v1 = Vec3::new(1., 2., 3.);
        assert_eq!(v1.x, 1.);
        assert_eq!(v1.y, 2.);
        assert_eq!(v1.z, 3.);
        let v2 = Vec3::new(1453., 2000., 30_000_000_000.);
        assert_eq!(v2.x, 1453.);
        assert_eq!(v2.y, 2000.);
        assert_eq!(v2.z, 30_000_000_000.);
    }

    #[test]
    fn creation2() {
        let v = Vec3::new(0.005, 0.02, 0.11234);
        assert_eq!(v.x, 0.005);
        assert_eq!(v.y, 0.02);
        assert_eq!(v.z, 0.11234);
    }

    #[test]
    fn creation3() {
        let v1 = Vec3::new(-200., -1., -555555.);
        assert_eq!(v1.x, -200.);
        assert_eq!(v1.y, -1.);
        assert_eq!(v1.z, -555555.);
        let v2 = Vec3::new(-5., -1., 22301.);
        assert_eq!(v2.x, -5.);
        assert_eq!(v2.y, -1.);
        assert_eq!(v2.z, 22301.);
        let v3 = Vec3::new(0.0009, -0.05, -200.005);
        assert_eq!(v3.x, 0.0009);
        assert_eq!(v3.y, -0.05);
        assert_eq!(v3.z, -200.005);
        let v3 = Vec3::new(200.002, -99.9991, 0.);
        assert_eq!(v3.x, 200.002);
        assert_eq!(v3.y, -99.9991);
        assert_eq!(v3.z, 0.);
        let v4 = Vec3::new(0., -0., 0.);
        assert_eq!(v4.x, 0.);
        assert_eq!(v4.y, -0.);
        assert_eq!(v4.z, 0.);
    }

    #[test]
    fn add1() {

    }

}
