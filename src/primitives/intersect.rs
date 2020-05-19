use crate::ray::{Ray, Hit};

pub trait Intersect: Send + Sync + std::fmt::Debug {
    fn intersection(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}