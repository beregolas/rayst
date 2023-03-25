use std::ops::{Index, IndexMut, Neg};
use crate::{vec_op};
use crate::math::ApproxEq;
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

    pub const X: Self = Vec3 {
        x: 1.,
        y: 0.,
        z: 0.,
    };

    pub const Y: Self = Vec3 {
        x: 0.,
        y: 1.,
        z: 0.,
    };

    pub const Z: Self = Vec3 {
        x: 0.,
        y: 0.,
        z: 1.,
    };

    pub const AXES: [Vec3; 3] = [Self::X, Self::Y, Self::Z];

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

impl ApproxEq for Vec3 {
    fn a_eq(&self, rhs: &Self) -> bool {
        self.x.a_eq(&rhs.x) && self.y.a_eq(&rhs.y) && self.z.a_eq(&rhs.z)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
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
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn min_vector(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }

    fn max_vector(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }

    fn min_component(&self) -> f32 {
        self.x.min(self.y.min(self.z))
    }

    fn max_component(&self) -> f32 {
        self.x.max(self.y.max(self.z))
    }
}

vec_op!(Vec3, +, x y z);
vec_op!(Vec3, -, x y z);
vec_op!(Vec3, *, x y z);
vec_op!(Vec3, /, x y z);

impl From<f32> for Vec3 {
    fn from(value: f32) -> Self {
        Vec3 { x: value, y: value, z: value }
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
    use crate::math::{EPSILON, Vec3, Vector};

    fn assert_float_eq(lhs: f32, rhs: f32) {
        assert!((lhs - rhs).abs() < EPSILON, "a: {} and b: {} are not close enough. allowed epsilon: {}, actual epsilon: {}", lhs, rhs, EPSILON, (lhs-rhs).abs());
    }

    fn assert_vec_eq(lhs: Vec3, rhs: Vec3) {
        for i in 0..=2 {
            assert_float_eq(lhs[i], rhs[i]);
        }
    }

    // this is only for more concise tests. It is not good syntax to use in the rest of the program
    macro_rules! v3 {
        ($x:expr, $y:expr, $z:expr) => {
            Vec3::new($x, $y, $z)
        }
    }

    #[test]
    fn creation1() {
        let v1 = v3!(1., 2., 3.);
        assert_float_eq(v1.x, 1.);
        assert_float_eq(v1.y, 2.);
        assert_float_eq(v1.z, 3.);
        let v2 = v3!(1453., 2000., 30_000_000_000.);
        assert_float_eq(v2.x, 1453.);
        assert_float_eq(v2.y, 2000.);
        assert_float_eq(v2.z, 30_000_000_000.);
    }

    #[test]
    fn creation2() {
        let v = v3!(0.005, 0.02, 0.11234);
        assert_float_eq(v.x, 0.005);
        assert_float_eq(v.y, 0.02);
        assert_float_eq(v.z, 0.11234);
    }

    #[test]
    fn creation3() {
        let v1 = v3!(-200., -1., -555555.);
        assert_float_eq(v1.x, -200.);
        assert_float_eq(v1.y, -1.);
        assert_float_eq(v1.z, -555555.);
        let v2 = v3!(-5., -1., 22301.);
        assert_float_eq(v2.x, -5.);
        assert_float_eq(v2.y, -1.);
        assert_float_eq(v2.z, 22301.);
        let v3 = v3!(0.0009, -0.05, -200.005);
        assert_float_eq(v3.x, 0.0009);
        assert_float_eq(v3.y, -0.05);
        assert_float_eq(v3.z, -200.005);
        let v3 = v3!(200.002, -99.9991, 0.);
        assert_float_eq(v3.x, 200.002);
        assert_float_eq(v3.y, -99.9991);
        assert_float_eq(v3.z, 0.);
        let v4 = v3!(0., -0., 0.);
        assert_float_eq(v4.x, 0.);
        assert_float_eq(v4.y, -0.);
        assert_float_eq(v4.z, 0.);
    }

    #[test]
    fn add1() {
        assert_vec_eq(v3!(1., 2., 3.) + v3!(10., 20., 100000.), v3!(11., 22., 100003.));
        assert_vec_eq(v3!(0., 0., 0.) + v3!(-10., -4., -100.), v3!(-10., -4., -100.));
        assert_vec_eq(v3!(100_000_000., 77., -500.) + v3!(0., -10., -74.), v3!(100_000_000., 67., -574.));
        assert_vec_eq(v3!(-123., -44., 100.) + v3!(123., -1., 9.), v3!(0., -45., 109.));
        assert_vec_eq(v3!(0.009, 0.7, 0.009) + v3!(0.001, 0.01, 0.09), v3!(0.01, 0.71, 0.099));
        assert_vec_eq(v3!(10., 99.999, 0.95) + v3!(20., -34.412, -0.03), v3!(30., 65.587, 0.92));
        assert_vec_eq(v3!(0.05, 0.9, 0.00006) + v3!(1., 10., 10000.), v3!(1.05, 10.9, 10000.00006));
        assert_vec_eq(v3!(0.05, 0.9, 0.00006) + v3!(1.1, 10.0009, 10000.55), v3!(1.15, 10.9009, 10_000.55));
    }

    #[test]
    fn add2() {
        let mut v = Vec3::ZERO;
        v += v3!(10., 20., 30.);
        assert_vec_eq(v, v3!(10., 20., 30.));
        v += v3!(-0.006, 0.9, 100.004);
        assert_vec_eq(v, v3!(9.994, 20.9, 130.004));
        v += v3!(-9.5, -80.9, 100000.);
        assert_vec_eq(v, v3!(0.494, -60., 100_130.01));
    }

    #[test]
    fn normalize() {
        assert_vec_eq(v3!(10., 0., 0.).normalized(), v3!(1., 0., 0.));
        assert_vec_eq(v3!(0., 10., 0.).normalized(), v3!(0., 1., 0.));
        assert_vec_eq(v3!(0., 0., 10.).normalized(), v3!(0., 0., 1.));
    }


    #[test]
    fn cross() {
        // right handed coordinate system
        assert_vec_eq(v3!(1., 0., 0.).cross(&v3!(0., 1., 0.)), v3!(0., 0., 1.));
    }


}
