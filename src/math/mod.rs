mod vec2;
mod vec3;
mod vec4;
mod macros;
mod vec;
mod matrix4x4;

pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;
pub use vec::Vector;

pub const EPSILON: f32 = 0.001;

pub trait ApproxEq {
    fn a_eq(&self, rhs: &Self) -> bool;
}

impl ApproxEq for f32 {
    fn a_eq(&self, rhs: &Self) -> bool {
        (self - rhs).abs() < EPSILON
    }
}