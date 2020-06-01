use crate::ray::{Ray, Hit};
extern crate nalgebra as na;
use na::{Vector3, Rotation3};
use crate::vecutil::VecUtil;
use rand::Rng;
use glam::Vec3;
use crate::lehmer::Lehmer;

pub struct ScatterRecord {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    return v - 2.0 * v.dot(&n) * n;
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}

pub trait Material: std::fmt::Debug + Send + Sync {
    #[inline]
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut Lehmer) -> Option<ScatterRecord>;
    #[inline]
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 { return Vec3::new(0.0, 0.0, 0.0); }
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
    fn scatter(&self, _ray: &Ray, hit: &Hit, rng: &mut Lehmer) -> Option<ScatterRecord> {
        let scatter_direction: Vector3<f64> = VecUtil::random_in_hemisphere(hit.normal, rng);
        
        return Some(ScatterRecord {
            scattered: Ray::new(hit.p, scatter_direction),
            attenuation: self.albedo,
        })
        
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        let mut tmp = fuzz;
        if tmp > 1.0 {
            tmp = 1.0;
        }
        Self { albedo, fuzz: tmp }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit, _rng: &mut Lehmer) -> Option<ScatterRecord> {
        
        let reflected = reflect(ray.direction.normalize(), hit.normal);
        
        Some(ScatterRecord {
            scattered: Ray::new(hit.p, reflected + (self.fuzz * VecUtil::random_in_unit_sphere(_rng))),
            attenuation: self.albedo,
        })
    }
}

#[derive(Debug)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx, }
    }
}

impl Material for Dielectric {
    
    #[inline(always)]
    fn scatter(&self, ray: &Ray, hit: &Hit, _rng: &mut Lehmer) -> Option<ScatterRecord> {
        let mut etai_over_etat: f64 = self.ref_idx;
        if hit.front_face {
            etai_over_etat = 1.0 / self.ref_idx;

            
        }
        let unit_direction = ray.direction.normalize();
        let mut cos_theta = (-unit_direction).dot(&hit.normal);
        if 1.0 < cos_theta
        {
            cos_theta = 1.0;
        }
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0
        {
            let refracted = VecUtil::refract(unit_direction, hit.normal, 1.0);

            return Some (ScatterRecord {
                scattered: Ray::new(hit.p, refracted),
                attenuation: Vec3::new(1.0, 1.0, 1.0),
            });            
        }

        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if _rng.gen_range(0.0, 1.0) < reflect_prob {
            let reflected = reflect(ray.direction.normalize(), hit.normal);
        
            return Some(ScatterRecord {
                scattered: Ray::new(hit.p, reflected),
                attenuation: Vec3::new(1.0, 1.0, 1.0),
            })
        }

        let refracted = VecUtil::refract(unit_direction, hit.normal, etai_over_etat);

        Some (ScatterRecord {
            scattered: Ray::new(hit.p, refracted),
            attenuation: Vec3::new(1.0, 1.0, 1.0),
        })
    }
}

#[derive(Debug)]
pub struct DiffuseLight {
    color: Vec3,
}

impl DiffuseLight {
    pub fn new(color: Vec3) -> Self {
        Self { color, }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, ray: &Ray, hit: &Hit, _rng: &mut Lehmer) -> Option<ScatterRecord> {
        return None;
    }

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        return self.color;
    }
}