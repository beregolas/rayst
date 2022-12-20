use crate::geometry::{Aabb, Geometry, Hit};
use crate::ray::Ray;

pub struct SimpleGroup {
    list: Vec<Box<dyn Geometry>>
}

impl SimpleGroup {
    pub fn new() -> Self {
        Self {
            list: Vec::new()
        }
    }

    pub fn push(&mut self, item: Box<dyn Geometry>) {
        self.list.push(item)
    }
}

impl Geometry for SimpleGroup {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.list.intersect(ray)
    }

    fn does_intersect(&self, ray: &Ray) -> bool {
        self.list.does_intersect(ray)
    }

    fn get_bounds(&self) -> Aabb {
        todo!()
    }
}
