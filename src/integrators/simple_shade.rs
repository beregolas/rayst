use crate::color::Color;
use crate::geometry::Geometry;
use crate::ray::Ray;

pub fn intersect(world: &impl Geometry, ray: &Ray) -> Color {

    let hit = world.intersect(ray);

    if let Some(hit) = hit  {
        Color::new(hit.normal.z, hit.normal.z, hit.normal.z)
    } else {
        Color::new(0., 0., 0.)
    }

}
