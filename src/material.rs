use crate::ray::{Ray, Hit};
use glam::Vec3;
use crate::vecutil::VecUtil;

pub struct ScatterRecord {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2.0 * v.dot(n) * n;
}

pub trait Material: std::fmt::Debug + Send + Sync{
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut rand::prelude::ThreadRng) -> Option<ScatterRecord>;
}

#[derive(Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit, rng: &mut rand::prelude::ThreadRng) -> Option<ScatterRecord> {
        let scatter_direction: Vec3 = hit.normal + VecUtil::random_in_unit_sphere(rng);
        
        return Some(ScatterRecord {
            scattered: Ray::new(hit.p, scatter_direction),
            attenuation: self.albedo,
        })
        
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit, _rng: &mut rand::prelude::ThreadRng) -> Option<ScatterRecord> {
        let reflected = reflect(ray.direction.normalize(), hit.normal);
        
        Some(ScatterRecord {
            scattered: Ray::new(hit.p, reflected),
            attenuation: self.albedo,
        })
    }
}