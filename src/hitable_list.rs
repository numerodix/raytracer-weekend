use crate::hitable::{Hitable, HitRecord};
use crate::ray::Ray;


pub struct HitableList {
    list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> Self {
        HitableList { list }
    }
}


impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut result = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for hitable in self.list.iter() {
            let mut opt_rec = hitable.hit(r, t_min, closest_so_far);
            if opt_rec.is_some() {
                let rec = opt_rec.take().unwrap();
                hit_anything = true;
                closest_so_far = rec.t;
                result = rec;
            }
        }

        match hit_anything {
            true => Some(result),
            false => None,
        }
    }
}