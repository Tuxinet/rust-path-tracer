use glam::{vec3, Vec3};
use crate::material::Material;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub inv_direction: Vec3,
}

impl Ray {
    #[inline(always)]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        let inv_direction = vec3(
            1.0 / direction.x(),
            1.0 / direction.y(),
            1.0 / direction.z(),
        );

        Self {
            origin,
            direction,
            inv_direction,
        }
    }

    #[inline(always)]
    pub fn at(&self, t: f32) -> Vec3 {
        return self.origin + t * self.direction;
    }
}

//#[derive(Default)]
pub struct Hit {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub material: Arc<dyn Material>,
    pub front_face: bool,
}

impl Hit {
    #[inline(always)]
    pub fn set_face_normal(&mut self, ray: &Ray) {
        self.front_face = ray.direction.dot(self.normal) < 0.0;
        if self.front_face {
            self.normal = self.normal;
        }
        else {
            self.normal = -self.normal;
        }
    }
}