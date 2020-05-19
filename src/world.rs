use crate::primitives::*;
use crate::ray::*;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct World {
    objects: Vec<Arc<dyn Intersect>>,
}

impl World {
    pub fn new() -> Self {
        let objects = Vec::<Arc<dyn Intersect>>::new();
        return Self {objects};
    }

    pub fn add_obj(&mut self, s: Arc<dyn Intersect>) {
        self.objects.push(s);
    }
}

impl Intersect for World {
    #[inline(always)]
    fn intersection(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {

        let mut ret = None;

        let mut closest: f64 = t_max;
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