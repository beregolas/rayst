use std::ops::{Index, IndexMut};
use crate::math::{ApproxEq, Vec3, Vector};
use crate::vec_op;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl ApproxEq for Vec4 {
    fn a_eq(&self, rhs: &Self) -> bool {
        self.x.a_eq(&rhs.x) && self.y.a_eq(&rhs.y)
            && self.z.a_eq(&rhs.z) && self.w.a_eq(&rhs.w)
    }
}

impl Vec4 {
    pub const ZERO: Self = Vec4 { x: 0., y: 0., z: 0., w: 0. };

    pub const ONE: Self = Vec4 { x: 1., y: 1., z: 1., w: 1. };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl Vector for Vec4 {
    fn normalize(&mut self) {
        let length: f32 = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
        self.w /= length;
    }

    fn normalized(&self) -> Self {
        let mut new_vector: Vec4 = *self;
        new_vector.normalize();
        new_vector
    }

    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan() || self.w.is_nan()
    }

    fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    fn min_vector(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
            w: self.w.min(rhs.w),
        }    
    }

    fn max_vector(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
            w: self.w.max(rhs.w),
        }    
    }

    fn min_component(&self) -> f32 {
        self.x.min(self.y.min(self.z.min(self.w)))
    }

    fn max_component(&self) -> f32 {
        self.x.max(self.y.max(self.z.max(self.w)))
    }
}


vec_op!(Vec4, +, x y z w);
vec_op!(Vec4, -, x y z w);
vec_op!(Vec4, *, x y z w);
vec_op!(Vec4, /, x y z w);


// Homogenous

impl From<Vec3> for Vec4 {
    fn from(value: Vec3) -> Self {
        Vec4 {
            x: value.x,
            y: value.y,
            z: value.z,
            w: 1.
        }
    }
}

impl From<Vec4> for Vec3 {
    fn from(value: Vec4) -> Self {
        Vec3 {
            x: value.x / value.w,
            y: value.y / value.w,
            z: value.z / value.w,
        }
    }
}

impl From<f32> for Vec4 {
    fn from(value: f32) -> Self {
        Vec4 { x: value, y: value, z: value, w: value }
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("index out of bounds"),
        }
    }
}


