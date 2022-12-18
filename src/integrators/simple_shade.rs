use crate::color::Color;
use crate::geometry::Geometry;
use crate::math::Vector;
use crate::ray::Ray;

pub fn intersect(world: &impl Geometry, ray: &Ray) -> Color {

    let hit = world.intersect(ray);
    let does_hit = world.does_intersect(ray);
    if hit.is_some() != does_hit {
        println!("FAILURE");
    }

    if let Some(hit) = hit  {
        Color::new(ray.direction.dot(&hit.normal), -ray.direction.dot(&hit.normal), 0.)
    } else {
        Color::new(0., 0., 0.)
    }

}
