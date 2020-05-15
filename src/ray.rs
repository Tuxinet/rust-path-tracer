use glam::{vec3, Vec3};
use crate::material::Material;
use std::boxed::Box;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub inv_direction: Vec3,
}

impl Ray {
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

    pub fn at(&self, t: f32) -> Vec3 {
        return self.origin + t * self.direction;
    }
}

//#[derive(Clone, Debug)]
pub struct Hit {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub material: Arc<dyn Material>,
}