use crate::color::Color;
use crate::geometry::Geometry;
use crate::ray::Ray;

fn intersect(world: &impl Geometry, ray: &Ray) -> Color {

    if world.does_intersect(ray) {
        Color::new(1., 1., 1.)
    } else {
        Color::new(0., 0., 0.)
    }

}
