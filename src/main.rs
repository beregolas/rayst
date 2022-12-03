#![allow(dead_code)]
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

use crate::camera::{Camera, PerspectiveCamera};
use crate::color::Color;
use crate::geometry::Sphere;
use crate::math::{Vec2, Vec3};

pub(crate) mod geometry;
pub(crate) mod math;
mod ray;
mod camera;
mod integrators;
mod color;

fn main() {
    let resolution = (200, 200);
    let mut img = ImageBuffer::new(resolution.0, resolution.1);


    let cam = PerspectiveCamera::new(Vec3::new(-30., 0., 0.), Vec3::new(1., 0., 0.), Vec3::new(0., 0., 1.), 1., 90.);
    let sphere = Sphere{ center: Vec3::new(0., 0., 0.), radius: 1.0 };

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let ray = cam.at(Vec2::new(x as f32 / resolution.0 as f32, y as f32 / resolution.1 as f32));
        let col = integrators::simple_shade::intersect(&sphere, &ray);
        *pixel = image::Rgb(col.to_u8());
    }

    // integrators::simple_shade::intersect(&sphere, &Ray {});

    img.save("test.png").unwrap();
}
