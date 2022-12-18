
pub trait Vector {
    fn normalize(&mut self);
    fn normalized(&self) -> Self;
    fn length(&self) -> f32;
    fn length_squared(&self) -> f32;
    fn is_nan(&self) -> bool;
    fn dot(&self, rhs: &Self) -> f32;
    fn min_vector(&self, rhs: &Self) -> Self;
    fn max_vector(&self, rhs: &Self) -> Self;
    fn min_component(&self) -> f32;
    fn max_component(&self) -> f32;
}