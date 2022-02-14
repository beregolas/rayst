mod aabb;

use crate::geometry::aabb::Aabb;
use crate::ray::Ray;

struct Hit {

}

pub trait Geometry {
    fn intersect(&self, r: Ray) -> Option<Hit>;
    fn does_intersect(&self, r: Ray) -> bool;
    fn bounds(&self) -> Aabb;
}