
pub trait Vector {
    fn normalize(&mut self);
    fn normalized(&self) -> Self;
    fn length(&self) -> f32;
    fn length_squared(&self) -> f32;
    fn is_nan(&self) -> bool;
    fn dot(&self, rhs: &Self) -> f32;
}