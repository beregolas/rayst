use crate::color::Color;
use crate::geometry::Geometry;
use crate::ray::Ray;

pub fn intersect(world: &impl Geometry, ray: &Ray) -> Color {

    if world.does_intersect(ray) {
        println!("hit!");
        Color::new(1., 1., 1.)
    } else {
        println!("miss!");
        Color::new(0., 0., 0.)
    }

}
