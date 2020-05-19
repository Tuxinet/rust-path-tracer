use glam::Vec3;
use rand::*;

pub struct VecUtil {

}

impl VecUtil {
    #[inline(always)]
    pub fn random(min: f32, max: f32, rng: &mut rand::prelude::ThreadRng) -> Vec3 {
        return Vec3::new(rng.gen_range(min, max), rng.gen_range(min, max), rng.gen_range(min, max));
    }

    #[inline(always)]
    pub fn random_in_unit_sphere(rng: &mut rand::prelude::ThreadRng) -> Vec3 {
        loop {
            let v = VecUtil::random(-1.0, 1.0, rng);
            if v.length_squared() <= 1.0
            {
                return v;
            }
        }
    }

    #[inline(always)]
    pub fn random_unit_vector(rng: &mut rand::prelude::ThreadRng) -> Vec3 {
        let a = rng.gen_range(0.0, 2.0*std::f32::consts::PI);
        let z = rng.gen_range(-1.0, 1.0);
        let r = ((1.0 - z*z) as f32).sqrt();
        return Vec3::new(r*a.cos(), r*a.sin(), z);
    }

    #[inline(always)]
    pub fn random_in_hemisphere(normal: Vec3, rng: &mut rand::prelude::ThreadRng) -> Vec3 {
        let in_unit_sphere = VecUtil::random_in_unit_sphere(rng);
        if (in_unit_sphere.dot(normal) > 0.0) { return in_unit_sphere; }
        else { return -in_unit_sphere; }
    }

    #[inline(always)]
    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = (-uv).dot(n);
        let r_out_parallel = etai_over_etat * (uv + cos_theta*n);
        let r_out_perp = -((1.0 - r_out_parallel.length_squared()).sqrt()) * n;
        return (r_out_parallel + r_out_perp);
    }

    #[inline(always)]
    pub fn random_in_unit_disk(rng: &mut rand::prelude::ThreadRng) -> Vec3 {
        loop {
            let p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}