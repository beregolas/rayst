use std::ops::{Index, IndexMut};
use crate::vec_op;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Vec2 { x: 0., y: 0. };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn normalize(&mut self) {
        let length: f32 = self.length();
        self.x /= length;
        self.y /= length;
    }

    pub fn normalized(&self) -> Self {
        let mut new_vector: Vec2 = *self;
        new_vector.normalize();
        new_vector
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn is_nan(&self) -> bool {
        self.x.is_nan() && self.y.is_nan()
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
}
vec_op!(Vec2, +, x y);
vec_op!(Vec2, -, x y);
vec_op!(Vec2, *, x y);
vec_op!(Vec2, /, x y);


impl From<f32> for Vec2 {
    fn from(value: f32) -> Self {
        Vec2 { x: value, y: value }
    }
}

impl Index<usize> for Vec2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("index out of bounds"),
        }
    }
}


