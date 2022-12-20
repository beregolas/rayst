use crate::geometry::Geometry;

pub mod simple_group;

pub trait Group {
    fn push(&mut self, item: Box<dyn Geometry>);
}