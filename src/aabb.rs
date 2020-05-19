extern crate nalgebra as na;
use na::{Vector3, Rotation3};
use crate::ray::{Ray, Hit};
use crate::primitives::Intersect;
use crate::material::*;
use std::sync::Arc;


pub struct aabb {
    min: Vector3<f64>,
    max: Vector3<f64>,
}

impl aabb {
    pub fn hit(ray: &Ray, tmin: f32, tmax: f32) {
        for a in 0..3 {
            //let invD = 1.0 / ray.direction
        }
    }
}