use std::f32::consts::PI;
use crate::color::Color;
use crate::integrators::Integrator;
use crate::math::Vector;
use crate::ray::Ray;
use crate::world::World;


pub struct RayTraceIntegrator<'a> {
    pub world: &'a World
}

impl Integrator for RayTraceIntegrator<'_> {

    fn li(&self, ray: &Ray) -> Color {
        let mut color = Color::BLACK;
        if let Some(hit) = self.world.geometry.intersect(ray){
            for light in self.world.lights.iter() {
                let (c, dir) = light.sample(hit.point, self.world);
                color += c * hit.normal.dot(&dir) / PI;
            }
        }
        color
    }
}
