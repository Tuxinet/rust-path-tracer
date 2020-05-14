use crate::primitives::*;
use crate::ray::*;
use glam::Vec3;

pub struct World {
    objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> Self {
        let objects = Vec::<Sphere>::new();
        return Self {objects};
    }

    pub fn add_obj(&mut self, s: Sphere) {
        //let s = Sphere::new(center, radius);
        self.objects.push(s);
    }
}

impl Intersect for World {
    fn intersection(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut hit: Hit = Hit {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0
        };

        let mut closest: f32 = t_max;
        let mut got_hit: bool = false;
        for s in self.objects.iter() {
            match s.intersection(ray, t_min, closest) {
                Some(hitR) => {
                    got_hit = true;
                    closest = hitR.t;
                    hit = hitR;
                },
                None => {

                }
            }
        }
        if got_hit {
            return Some(hit);
        }
        None
    }
}