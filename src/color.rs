

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
}