use crate::primitives::*;
use crate::ray::*;

#[derive(Clone, Debug)]
pub struct World {
    objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> Self {
        let objects = Vec::<Sphere>::new();
        return Self {objects};
    }

    pub fn add_obj(&mut self, s: Sphere) {
        self.objects.push(s);
    }
}

impl Intersect for World {
    fn intersection(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {

        let mut ret = None;

        let mut closest: f32 = t_max;
        let mut got_hit: bool = false;
        for s in self.objects.iter() {
            match s.intersection(ray, t_min, closest) {
                Some(hit_r) => {
                    got_hit = true;
                    closest = hit_r.t;
                    ret = Some(hit_r);
                },
                None => {

                }
            }
        }
        if got_hit {
            return ret;
        }
        None
    }
}