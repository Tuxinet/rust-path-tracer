use glam::Vec3;
use crate::ray::Ray;

#[derive(Clone, Debug)]
pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);

        return Self {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        return Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin);
    }
}