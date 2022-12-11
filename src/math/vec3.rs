use std::ops::{Index, IndexMut};
use crate::{vec_access, vec_op};
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

    vec_access!(x y z);

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
    fn creation() {
        let v1 = Vec3::new(1., 2., 3.);
        println!("{:?}", v1.xyz());
    }

}
