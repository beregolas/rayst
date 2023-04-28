use crate::geometry::{Aabb, Geometry, Hit};
use crate::groups::{Group, GroupContent};
use crate::ray::Ray;

#[derive(Default)]
pub struct SimpleGroup<'a> {
    list: Vec<GroupContent<'a>>
}

impl<'a> SimpleGroup<'a> {
    pub fn new() -> Self {
        Self {
            list: Vec::new()
        }
    }
}

impl<'a> Group for SimpleGroup<'a> {
    fn push(&mut self, item: GroupContent) {
        self.list.push(item)
    }
}

impl<'a> Geometry for SimpleGroup<'a> {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let mut hit: Option<Hit> = None;
        for g in &self.list {
            if let Some(new_hit) = g.item.intersect(ray) {
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
