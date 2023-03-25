use std::ops::{Index, IndexMut, Mul};
use crate::math::{Vec3, Vec4};

pub struct Mat4 {
    x_axis: Vec4,
    y_axis: Vec4,
    z_axis: Vec4,
    w_axis: Vec4,
}

impl Mat4 {

    fn new(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis
        }
    }

    pub const ZERO: Self = Mat4 {
        x_axis: Vec4::ZERO,
        y_axis: Vec4::ZERO,
        z_axis: Vec4::ZERO,
        w_axis: Vec4::ZERO,
    };

    pub const ONE: Self = Mat4 {
        x_axis: Vec4::ONE,
        y_axis: Vec4::ONE,
        z_axis: Vec4::ONE,
        w_axis: Vec4::ONE,
    };

    fn translate(by: Vec3) -> Self {
        Mat4 {
            x_axis: Vec4::new(1., 0., 0., by.x),
            y_axis: Vec4::new(0., 1., 0., by.y),
            z_axis: Vec4::new(0., 0., 1., by.z),
            w_axis: Vec4::new(0., 0., 0., 1.),
        }
    }

    fn scale(by: Vec3) -> Self {
        Mat4 {
            x_axis: Vec4::new(by.x, 0., 0., 0.),
            y_axis: Vec4::new(0., by.y, 0., 0.),
            z_axis: Vec4::new(0., 0., by.z, 0.),
            w_axis: Vec4::new(0., 0., 0., 1.),
        }
    }

    fn mirror_x() -> Self {
        Mat4 {
            x_axis: Vec4::new(-1., 0., 0., 0.),
            y_axis: Vec4::new(0., 1., 0., 0.),
            z_axis: Vec4::new(0., 0., 1., 0.),
            w_axis: Vec4::new(0., 0., 0., 1.),
        }
    }

    fn mirror_y() -> Self {
        Mat4 {
            x_axis: Vec4::new(1., 0., 0., 0.),
            y_axis: Vec4::new(0., -1., 0., 0.),
            z_axis: Vec4::new(0., 0., 1., 0.),
            w_axis: Vec4::new(0., 0., 0., 1.),
        }
    }

    fn mirror_z() -> Self {
        Mat4 {
            x_axis: Vec4::new(1., 0., 0., 0.),
            y_axis: Vec4::new(0., 1., 0., 0.),
            z_axis: Vec4::new(0., 0., -1., 0.),
            w_axis: Vec4::new(0., 0., 0., 1.),
        }
    }

    fn mirror_0() -> Self {
        Mat4 {
            x_axis: Vec4::new(-1., 0., 0., 0.),
            y_axis: Vec4::new(0., -1., 0., 0.),
            z_axis: Vec4::new(0., 0., -1., 0.),
            w_axis: Vec4::new(0., 0., 0., 1.),
        }
    }

    fn shear(x_y: f32, x_z: f32, y_z: f32, y_x: f32, z_x: f32, z_y: f32) -> Self {
        Mat4 {
            x_axis: Vec4::new(1., x_y, x_z, 0.),
            y_axis: Vec4::new(y_x, 1., y_z, 0.),
            z_axis: Vec4::new(z_x, z_y, 1., 0.),
            w_axis: Vec4::new(0., 0., 0., 1.),
        }
    }

    fn rotate_x(by: f32) -> Self {
        Mat4 {
            x_axis: Vec4::new(1., 0., 0., 0.),
            y_axis: Vec4::new(0., by.cos(), -by.sin(), 0.),
            z_axis: Vec4::new(0., by.cos(), by.sin(), 0.),
            w_axis: Vec4::new(0., 0., 0., 1.),
        }
    }

    fn rotate_y(by: f32) -> Self {
        Mat4 {
            x_axis: Vec4::new(by.cos(), 0., by.sin(), 0.),
            y_axis: Vec4::new(0., 1., 0., 0.),
            z_axis: Vec4::new(-by.sin(), 0., by.cos(), 0.),
            w_axis: Vec4::new(0., 0., 0., 1.),
        }
    }

    fn rotate_z(by: f32) -> Self {
        Mat4 {
            x_axis: Vec4::new(by.cos(), -by.sin(), 0., 0.),
            y_axis: Vec4::new(by.sin(), by.cos(), 0., 0.),
            z_axis: Vec4::new(0., 0., -1., 0.),
            w_axis: Vec4::new(0., 0., 0., 1.),
        }
    }

}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        let mut res = Self::ZERO;
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    res[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        res
    }
}

impl Index<usize> for Mat4 {
    type Output = Vec4;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x_axis,
            1 => &self.y_axis,
            2 => &self.z_axis,
            3 => &self.w_axis,
            _ => panic!("index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x_axis,
            1 => &mut self.y_axis,
            2 => &mut self.z_axis,
            3 => &mut self.w_axis,
            _ => panic!("index out of bounds"),
        }
    }
}
