use crate::geometry::{Aabb, Geometry, Hit};
use crate::ray::Ray;

pub struct SimpleGroup {

}

impl Geometry for SimpleGroup {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        todo!()
    }

    fn does_intersect(&self, ray: &Ray) -> bool {
        todo!()
    }

    fn get_bounds(&self) -> Aabb {
        todo!()
    }
}
