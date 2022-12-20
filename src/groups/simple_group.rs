use crate::geometry::{Aabb, Geometry, Hit};
use crate::groups::Group;
use crate::ray::Ray;

#[derive(Default)]
pub struct SimpleGroup {
    list: Vec<Box<dyn Geometry>>
}

impl SimpleGroup {
    pub fn new() -> Self {
        Self {
            list: Vec::new()
        }
    }
}

impl Group for SimpleGroup {
    fn push(&mut self, item: Box<dyn Geometry>) {
        self.list.push(item)
    }
}

impl Geometry for SimpleGroup {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut hit: Option<Hit> = None;
        for g in &self.list {
            if let Some(new_hit) = g.intersect(ray) {
                if let Some(old_hit) = hit {
                    if new_hit.distance < old_hit.distance {
                        hit = Some(new_hit);
                    }
                } else {
                    hit = Some(new_hit);
                }
            }
        }
        hit
    }

    fn does_intersect(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }

    fn get_bounds(&self) -> Aabb {
        todo!()
    }
}
