use std::fmt::Debug;
use crate::color::Color;
use crate::geometry::Geometry;
use crate::lights::LightSource;

use crate::ray::Ray;

fn make_color(value: f32) -> f32 {
    match value {
        x if x < 0. => -x,
        x => x / 2.,
    }
}

fn normal_coloring(world: &impl Geometry, ray: &Ray) -> Color {
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

fn distance_coloring(world: &impl Geometry, ray: &Ray) -> Color {
    let hit = world.intersect(ray);
    if let Some(hit) = hit {
        Color::WHITE * 200. / hit.distance
    } else {
        Color::BLACK
    }
}


fn lambertian(world: &impl Geometry, lights: Vec<Box<dyn LightSource>>, ray: &Ray) -> Color {
    let hit = world.intersect(ray);
    if let Some(hit) = hit {
        let mut color = Color::BLACK;
        for lightsource in lights {
            // add incident light
            color += lightsource.sample(hit.point, world);
        }
        color
    } else {
        Color::new(0., 0., 0.)
    }
}

pub fn intersect(world: &impl Geometry, lights: Vec<Box<dyn LightSource>>, ray: &Ray) -> Color {
   // normal_coloring(world, ray)
   lambertian(world, lights, ray)
   // distance_coloring(world, ray)
}
