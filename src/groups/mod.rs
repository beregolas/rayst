use crate::geometry::{Geometry, Sphere};
use crate::materials::Material;

pub mod simple_group;

pub struct GroupContent<'a> {
    item: Box<dyn Geometry>,
    material: Option<&'a Box<dyn Material>>
}

impl<'a> GroupContent<'a> {
    pub(crate) fn new(item: Box<dyn Geometry>, material: Option<&Box<dyn Material>>) -> GroupContent {
        GroupContent {
            item,
            material
        }
    }
}

pub trait Group: Geometry {
    fn push(&mut self, item: GroupContent);
}