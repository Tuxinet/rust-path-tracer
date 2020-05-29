extern crate nalgebra as na;
use na::{Vector3, distance, distance_squared};
use crate::ray::{Ray, Hit};
use crate::primitives::Intersect;
use crate::material::*;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Arc<dyn Material>) -> Self {
        Self { center, radius, material }
    }
}

impl Intersect for Sphere {
    #[inline]
    fn intersection(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        let b = oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-b - root) / a;
            if temp < t_max && temp > t_min {
                let mut hit = Hit {
                    t: temp,
                    p: ray.at(temp),
                    normal: (ray.at(temp) - self.center) / self.radius,
                    material: self.material.clone(),
                    front_face: true,
                };
                hit.set_face_normal(ray);
                return Some(hit);
            }

            temp = (-b + root) / a;
            if temp < t_max && temp > t_min {
                let mut hit = Hit {
                    t: temp,
                    p: ray.at(temp),
                    normal: (ray.at(temp) - self.center) / self.radius,
                    material: self.material.clone(),
                    front_face: false,
                };
                hit.set_face_normal(ray);
                return Some(hit);
            }
        }

        None
    }
}