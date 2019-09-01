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
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hitable in self.list.iter() {
            let opt_rec = hitable.hit(r, t_min, closest_so_far);
            if opt_rec.is_some() {
                hit_record = opt_rec;
                closest_so_far = hit_record.as_ref().unwrap().t;
            }
        }

        hit_record
    }
}