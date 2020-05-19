use glam::Vec3;
use crate::ray::Ray;
use crate::vecutil::VecUtil;
use rand::prelude::ThreadRng;

#[derive(Clone, Debug)]
pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, look_at: Vec3, up:Vec3, vfov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Self {
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (origin - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

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
    pub fn get_ray(&self, s: f32, t: f32, rng: &mut ThreadRng) -> Ray {
        let rd = self.lens_radius * VecUtil::random_in_unit_disk(rng);
        let offset = self.u * rd.x() + self.v * rd.y();
        
        return Ray::new(self.origin + offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset);
    }
}