use glam::Vec3;
use rand::*;

pub struct VecUtil {

}

impl VecUtil {
    pub fn random(min: f32, max: f32, rng: &mut rand::prelude::ThreadRng) -> Vec3 {
        return Vec3::new(rng.gen_range(min, max), rng.gen_range(min, max), rng.gen_range(min, max));
    }

    pub fn random_in_unit_sphere(rng: &mut rand::prelude::ThreadRng) -> Vec3 {
        while true {
            let v = VecUtil::random(-1.0, 1.0, rng);
            if (v.length_squared() <= 1.0)
            {
                return v;
            }
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }
}