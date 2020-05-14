use crate::ray::{Ray, Hit};


pub trait Intersect: Send + Sync {
    fn intersection(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}