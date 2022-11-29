// use crate::ray::Ray;
//
//
// pub trait Camera {
//     // input: coords from (0,0) to (1,1)
//     fn at(&self, coords: Vec2) -> Ray;
// }
//
//
// fn prepare_input(coords: Vec2) -> Vec2 {
//     (coords - 0.5) * 2.
// }
//
//
// #[derive(Debug)]
// pub struct OrthographicCamera {
//     // Forward is normalized
//     // up and right are scaled to half width & height
//     origin: Vec3,
//     forward: Vec3,
//     up: Vec3,
//     right: Vec3
// }
//
// impl OrthographicCamera {
//     pub fn new(origin: Vec3, forward: Vec3, up: Vec3, size: Vec2) -> Self {
//         let f = forward.normalize();
//         let u = up.normalize();
//         debug_assert!(!f.is_nan());
//         debug_assert!(!u.is_nan());
//         let r = f.cross(u).normalize();
//         Self {
//             origin,
//             forward: f,
//             right: r * size.x,
//             up: r.cross(f).normalize() * size.y
//         }
//     }
// }
//
// impl Camera for OrthographicCamera {
//     fn at(&self, coords: Vec2) -> Ray {
//         let c = prepare_input(coords);
//         Ray::new(
//             self.up * -c.y + self.right * c.x,
//             self.forward
//         )
//     }
// }
//
// #[derive(Debug)]
// pub struct PerspectiveCamera {
//     // Forward is normalized
//     // up and right are scaled to half width & height of image plane at distance 1
//     origin: Vec3,
//     forward: Vec3,
//     up: Vec3,
//     right: Vec3
// }
//
// impl PerspectiveCamera {
//     pub fn new(origin: Vec3, forward: Vec3, up: Vec3, aspect: f32, fov: f32) -> Self {
//         let f = forward.normalize();
//         let u = up.normalize();
//         debug_assert!(!f.is_nan());
//         debug_assert!(!u.is_nan());
//         let r = f.cross(u).normalize();
//         // opening angle between forward and view border
//         let alpha = fov / 180. * std::f32::consts::PI / 2.;
//         let size = Vec2::new(alpha.tan(), alpha.tan() / aspect);
//         return Self {
//             origin,
//             forward: f,
//             right: r * size.x,
//             up: u * size.y
//         }
//     }
// }
//
// impl Camera for PerspectiveCamera {
//     fn at(&self, coords: Vec2) -> Ray {
//         let c = prepare_input(coords);
//         Ray::new(self.origin, self.forward + self.up * -c.y + self.right * c.x)
//     }
// }
//
// #[cfg(test)]
// mod camera_tests {
//
//     #[cfg(test)]
//     mod perspective_tests {
//         use bevy_math::{Vec2, Vec3};
//         use crate::camera::{Camera, PerspectiveCamera};
//
//         #[test]
//         fn create_success() {
//             let c1 = PerspectiveCamera::new(
//                 Vec3::new(10., 10., 0.),
//                 Vec3::new(-1., -1., 0.),
//                 Vec3::new(1., 0., 0.),
//                 4. / 3.,
//                 90.
//             );
//             assert_eq!(c1.origin, Vec3::new(10., 10., 0.));
//             assert_eq!(c1.forward, Vec3::new(-1., -1., 0.).normalize());
//         }
//
//         #[test]
//         fn project() {
//             let c1 = PerspectiveCamera::new(
//                 Vec3::new(10., 10., 0.),
//                 Vec3::new(-1., -1., 0.),
//                 Vec3::new(1., 0., 0.),
//                 4. / 3.,
//                 90.
//             );
//             assert!(
//                 (c1.at(Vec2::new(0.5, 0.5)).at(Vec3::new(10., 10., 0.).length()) - Vec3::new(0., 0., 0.)).length() < 0.00005
//             )
//         }
//     }
//
//     #[cfg(test)]
//     mod orthogonal_tests {
//         #[test]
//         fn create_success() {
//
//         }
//     }
//
// }
