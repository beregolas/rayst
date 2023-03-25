#![allow(dead_code)]
use image::{ImageBuffer};

use crate::camera::{Camera, PerspectiveCamera};
use crate::color::Color;
use crate::geometry::{Aabb, Sphere, Triangle};
use crate::groups::Group;
use crate::groups::simple_group::SimpleGroup;
use crate::integrators::Integrator;
use crate::integrators::ray_trace::RayTraceIntegrator;
use crate::lights::point::PointLight;
use crate::math::{Vec2, Vec3};
use crate::world::World;

pub mod geometry;
pub mod math;
pub mod ray;
pub mod camera;
pub mod integrators;
pub mod color;
pub mod groups;
pub mod lights;
pub mod materials;
pub mod world;

fn main() {
    let resolution = (900, 900);
    let mut img = ImageBuffer::new(resolution.0, resolution.1);

    let mut world = World{
        geometry: Box::new(SimpleGroup::new()),
        materials: vec![],
        lights: vec![],
    };

    let cam = PerspectiveCamera::new(Vec3::new(278., 273., -800.), Vec3::new(0., 0., 1.), Vec3::new(0., 1., 0.), 1., 45.);
    // let cam = PerspectiveCamera::new(Vec3::new(2., 2., 2.), Vec3::new(-1., -1., -1.), Vec3::new(0., 0., 1.), 1., 90.);
    // let cam = OrthographicCamera::new(Vec3::new(-10., 0., 0.), Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.), Vec2::new(2., 2.));
    let sphere1 = Sphere::new(Vec3::new(300., 0., 200.), 100.);
    world.geometry.push(Box::new(sphere1));
    let box1 = Aabb::new(Vec3::new(100., 500., 300.), Vec3::new(400., 400., 400.));
    world.geometry.push(Box::new(box1));
    build_cornell_box(&mut *world.geometry);

    world.lights.push(Box::new(PointLight::new(Vec3::new(250., 400., 150.), Color::new(200000., 150000., 100000.))));

    let mut integrator = RayTraceIntegrator {
        world: &world
    };


    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let ray = cam.at(Vec2::new(x as f32 / resolution.0 as f32, y as f32 / resolution.1 as f32));
        let col = integrator.li(&ray);
        *pixel = image::Rgb(col.to_u8());
    }

    // integrators::simple_shade::intersect(&sphere, &Ray {});

    img.save("test.png").unwrap();
}




fn build_cornell_box(world: &mut dyn Group) {
    // FLOOR
    world.push(Box::new(Triangle::new(Vec3::new(0., 0., 0.), Vec3::new(552.8, 0., 0.), Vec3::new(0., 0., 559.2))));
    world.push(Box::new(Triangle::new(Vec3::new(0., 0., 559.2),Vec3::new(552.8, 0., 0.), Vec3::new(549.6, 0., 559.2))));
    // CEILING
    world.push(Box::new(Triangle::new(Vec3::new(0., 548.8, 0.), Vec3::new(0., 548.8, 559.2), Vec3::new(556., 548.8, 0.))));
    world.push(Box::new(Triangle::new(Vec3::new(0., 548.8, 559.2),Vec3::new(556., 548.8, 559.2), Vec3::new(556., 548.8, 0.))));
    // BACK WALL
    world.push(Box::new(Triangle::new(Vec3::new(556., 548.8, 559.2), Vec3::new(0., 548.8, 559.2), Vec3::new(549.6, 0., 559.2))));
    world.push(Box::new(Triangle::new(Vec3::new(0., 548.8, 559.2), Vec3::new(0., 0., 559.2), Vec3::new(549.6, 0., 559.2))));
   // RIGHT WALL
    world.push(Box::new(Triangle::new(Vec3::new(0., 548.8, 559.2), Vec3::new(0., 548.8, 0.), Vec3::new(0., 0., 559.2))));
    world.push(Box::new(Triangle::new(Vec3::new(0., 548.8, 0.), Vec3::new(0., 0., 0.), Vec3::new(0., 0., 559.2))));
    // LEFT WALL
    world.push(Box::new(Triangle::new(Vec3::new(556., 548.8, 0.), Vec3::new(556., 548.8, 559.2), Vec3::new(552.8, 0., 0.))));
    world.push(Box::new(Triangle::new(Vec3::new(556., 548.8, 559.2), Vec3::new(549.6, 0., 559.2), Vec3::new(552.8, 0., 0.))));

}