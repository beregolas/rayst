use std::ops::{Index, IndexMut};
use crate::math::{ApproxEq, Vector};
use crate::vec_op;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl ApproxEq for Vec2 {
    fn a_eq(&self, rhs: &Self) -> bool {
        self.x.a_eq(&rhs.x) && self.y.a_eq(&rhs.y)
    }
}

impl Vec2 {
    pub const ZERO: Self = Vec2 { x: 0., y: 0. };

    pub const DOWN: Self = Vec2 { x: 0., y: 1. };

    pub const UP: Self = Vec2 { x: 0., y: -1. };

    pub const RIGHT: Self = Vec2 { x: 1., y: 0. };

    pub const LEFT: Self = Vec2 { x: -1., y: 0. };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Vector for Vec2 {
    fn normalize(&mut self) {
        let length: f32 = self.length();
        self.x /= length;
        self.y /= length;
    }

    fn normalized(&self) -> Self {
        let mut new_vector: Vec2 = *self;
        new_vector.normalize();
        new_vector
    }

    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    fn min_vector(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }

    fn max_vector(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    fn min_component(&self) -> f32 {
        self.x.min(self.y)
    }

    fn max_component(&self) -> f32 {
        self.x.max(self.y)
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


