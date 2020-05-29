extern crate nalgebra as na;
use na::{Vector3, Rotation3};
use rand::*;

pub struct VecUtil {

}

impl VecUtil {
    #[inline]
    pub fn random(min: f64, max: f64, rng: &mut rand::prelude::SmallRng) -> Vector3<f64> {
        return Vector3::<f64>::new(rng.gen_range(min, max), rng.gen_range(min, max), rng.gen_range(min, max));
    }

    #[inline]
    pub fn random_in_unit_sphere(rng: &mut rand::prelude::SmallRng) -> Vector3<f64> {
        loop {
            let v = VecUtil::random(-1.0, 1.0, rng);
            if v.norm_squared() <= 1.0
            {
                return v;
            }
        }
    }

    #[inline]
    pub fn random_unit_vector(rng: &mut rand::prelude::SmallRng) -> Vector3<f64> {
        let a = rng.gen_range(0.0, 2.0*std::f64::consts::PI);
        let z = rng.gen_range(-1.0, 1.0);
        let r = ((1.0 - z*z) as f64).sqrt();
        return Vector3::<f64>::new(r*a.cos(), r*a.sin(), z);
    }

    #[inline]
    pub fn random_in_hemisphere(normal: Vector3<f64>, rng: &mut rand::prelude::SmallRng) -> Vector3<f64> {
        let in_unit_sphere = VecUtil::random_in_unit_sphere(rng);
        if in_unit_sphere.dot(&normal) > 0.0 { return in_unit_sphere; }
        else { return -in_unit_sphere; }
    }

    #[inline]
    pub fn refract(uv: Vector3<f64>, n: Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
        let cos_theta = (-uv).dot(&n);
        let r_out_parallel = etai_over_etat * (uv + cos_theta*n);
        let r_out_perp = -((1.0 - r_out_parallel.norm_squared()).sqrt()) * n;
        return r_out_parallel + r_out_perp;
    }

    #[inline]
    pub fn random_in_unit_disk(rng: &mut rand::prelude::SmallRng) -> Vector3<f64> {
        loop {
            let p = Vector3::<f64>::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
            if p.norm_squared() < 1.0 {
                return p;
            }
        }
    }
}