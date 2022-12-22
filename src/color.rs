use crate::vec_op;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color {
            r,
            g,
            b,
        }
    }

    pub fn to_u8(&self) -> [u8; 3] {
        [
            (self.r.clamp(0., 1.) * 255.) as u8,
            (self.g.clamp(0., 1.) * 255.) as u8,
            (self.b.clamp(0., 1.) * 255.) as u8,
        ]
    }

    pub const BLACK: Self = Color{
        r: 0.,
        g: 0.,
        b: 0.
    };
    pub const WHITE: Self = Color{
        r: 1.,
        g: 1.,
        b: 1.
    };
    pub const RED: Self = Color{
        r: 1.,
        g: 0.,
        b: 0.
    };
    pub const GREEN: Self = Color{
        r: 0.,
        g: 1.,
        b: 0.
    };
    pub const BLUE: Self = Color{
        r: 0.,
        g: 0.,
        b: 1.
    };
}

vec_op!(Color, +, r g b);
vec_op!(Color, -, r g b);
vec_op!(Color, *, r g b);
vec_op!(Color, /, r g b);