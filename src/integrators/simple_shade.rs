use crate::color::Color;
use crate::geometry::Geometry;
use crate::math::{Vec3, Vector};
use crate::ray::Ray;

fn make_color(value: f32) -> f32 {
    match value {
        0. => 0.,
        x if x < 0. => -x,
        x => x / 2.,
    }
}

pub fn intersect(world: &impl Geometry, ray: &Ray) -> Color {

    let hit = world.intersect(ray);
    let does_hit = world.does_intersect(ray);
    if hit.is_some() != does_hit {
        println!("FAILURE");
    }

    if let Some(hit) = hit  {
        Color::new(make_color(hit.normal.x), make_color(hit.normal.y), make_color(hit.normal.z))
    } else {
        Color::new(0., 0., 0.)
    }

}
