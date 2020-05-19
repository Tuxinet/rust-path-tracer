extern crate nalgebra as na;
use na::{Vector3, Rotation3};
use crate::ray::Ray;
use crate::vecutil::VecUtil;
use rand::prelude::ThreadRng;

#[derive(Clone, Debug)]
pub struct Camera {
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    origin: Vector3<f64>,
    lens_radius: f64,
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,
}

impl Camera {
    pub fn new(origin: Vector3<f64>, look_at: Vector3<f64>, up: Vector3<f64>, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (origin - look_at).normalize();
        let u = up.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_dist * w;

        return Self {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
        }
    }

    #[inline(always)]
    pub fn get_ray(&self, s: f64, t: f64, rng: &mut ThreadRng) -> Ray {
        let rd = self.lens_radius * VecUtil::random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;
        
        return Ray::new(self.origin + offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset);
    }
}