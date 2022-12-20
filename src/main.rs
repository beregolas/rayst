#![allow(dead_code)]
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

use crate::camera::{Camera, OrthographicCamera, PerspectiveCamera};
use crate::color::Color;
use crate::geometry::{Aabb, Sphere, Triangle};
use crate::math::{Vec2, Vec3};

pub mod geometry;
pub mod math;
pub mod ray;
pub mod camera;
pub mod integrators;
pub mod color;
pub mod groups;

fn main() {
    let resolution = (200, 200);
    let mut img = ImageBuffer::new(resolution.0, resolution.1);


    let cam = PerspectiveCamera::new(-Vec3::new(2., 2., 2.), -Vec3::new(-1., -1., -1.), -Vec3::new(0., 0., 1.), 1., 90.);
    // let cam = PerspectiveCamera::new(Vec3::new(2., 2., 2.), Vec3::new(-1., -1., -1.), Vec3::new(0., 0., 1.), 1., 90.);
    // let cam = OrthographicCamera::new(Vec3::new(-10., 0., 0.), Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.), Vec2::new(2., 2.));
    // let world = Sphere::new(Vec3::new(0., 0., 0.), 1.);
    // let world = Aabb::new(Vec3::new(0., 0., 0.), Vec3::new(1., 1., 1.));
    let world = Triangle::new(Vec3::new(0., 0., 0.), Vec3::new(0., 2., 0.), Vec3::new(2., 0., 0.));

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let ray = cam.at(Vec2::new(x as f32 / resolution.0 as f32, y as f32 / resolution.1 as f32));
        let col = integrators::simple_shade::intersect(&world, &ray);
        *pixel = image::Rgb(col.to_u8());
    }

    // integrators::simple_shade::intersect(&sphere, &Ray {});

    img.save("test.png").unwrap();
}
