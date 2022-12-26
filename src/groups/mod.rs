use crate::geometry::Geometry;

pub mod simple_group;

pub trait Group: Geometry {
    fn push(&mut self, item: Box<dyn Geometry>);
}