use std::ops::{Index, IndexMut};
use crate::{vec_op};

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

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn normalize(&mut self) {
        let length: f32 = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    pub fn normalized(&self) -> Self {
        let mut new_vector: Vec3 = *self;
        new_vector.normalize();
        new_vector
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn is_nan(&self) -> bool {
        self.x.is_nan() && self.y.is_nan() && self.z.is_nan()
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
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
    use crate::math::Vec3;

    macro_rules! creation_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
            #[test]
            fn $name() {
                let (x, y, z) = $value;
                let v = Vec3::new(x, y, z);
                assert_eq!(v.x, x);
                assert_eq!(v.y, y);
                assert_eq!(v.z, z);
            }
            )*
        }
    }
    creation_tests! {
        create_1: (0., 0., 0.),
        create_2: (-15., 4324234., 1.123_123),
        create_3: (0.1, 0.2, 0.3),
        create_4: (-30., -20., -10.),
        create_5: (1233.1233, 3211.0001, 1233.0000002),
    }

    #[test]
    fn add_1() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 4., 4.);
        let v3 = Vec3::new(-0.5, -0.3, -0.9);
        let v4 = Vec3::new(122222., 122222., 122222.);
        let v5 = Vec3::new(0.0005, 100.9, 31.0003);
        assert_eq!(Vec3::new(5., 6., 7.), v1 + v2);
        assert_eq!(Vec3::new(5., 6., 7.), v2 + v1);
        assert_eq!(Vec3::new(0.5, 1.7, 2.1), v1 + v3);
        assert_eq!(Vec3::new(122223., 122224., 122225.), v1 + v4);
        assert_eq!(Vec3::new(-0.4995, 100.6, 30.1003), v5 + v3);
    }

    #[test]
    fn add_2() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 4., 4.);
        let v3 = Vec3::new(-0.5, -0.3, -0.9);
        let v4 = Vec3::new(122222., 122222., 122222.);
        assert_eq!(Vec3::new(3., 4., 5.), v1 + 2.);
        assert_eq!(Vec3::new(-3.5, -3.5, -3.5), v2 + -7.5);
        assert_eq!(Vec3::new(100.5, 100.7, 100.1), v3 + 101.);
        assert_eq!(Vec3::new(129999., 129999., 129999.), v4 + 7777.);
    }

    #[test]
    fn add_3() {
        let v1 =

    }

    #[test]
    fn sub_1() {

    }
}
