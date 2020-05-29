extern crate nalgebra as na;
use na::{Vector3, Rotation3};
use crate::material::Material;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
    pub inv_direction: Vector3<f64>,
}

impl Ray {
    #[inline]
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        let inv_direction = Vector3::<f64>::new(
            1.0 / direction.x,
            1.0 / direction.y,
            1.0 / direction.z,
        );

        Self {
            origin,
            direction,
            inv_direction,
        }
    }

    #[inline]
    pub fn at(&self, t: f64) -> Vector3<f64> {
        return self.origin + t * self.direction;
    }
}

//#[derive(Default)]
pub struct Hit {
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub material: Arc<dyn Material>,
    pub front_face: bool,
}

impl Hit {
    #[inline]
    pub fn set_face_normal(&mut self, ray: &Ray) {
        self.front_face = ray.direction.dot(&self.normal) < 0.0;
        if self.front_face {
            self.normal = self.normal;
        }
        else {
            self.normal = -self.normal;
        }
    }
}