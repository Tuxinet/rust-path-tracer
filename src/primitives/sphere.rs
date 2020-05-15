use glam::{Vec3};
use crate::ray::{Ray, Hit};
use crate::primitives::Intersect;
use crate::material::*;
use std::sync::Arc;
use std::boxed::Box;

//#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
        Self { center, radius, material }
    }
}

impl Intersect for Sphere {
    fn intersection(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-b - root) / a;
            if (temp < t_max && temp > t_min) {
                let hit = Hit {
                    t: temp,
                    p: ray.at(temp),
                    normal: (ray.at(temp) - self.center) / self.radius,
                    material: self.material.clone(),
                };
                return Some(hit);
            }

            temp = (-b + root) / a;
            if (temp < t_max && temp > t_min) {
                return Some( Hit {
                    t: temp,
                    p: ray.at(temp),
                    normal: (ray.at(temp) - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
        }

        None
    }
}