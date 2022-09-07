use crate::ray_tracing::ray::Ray;

use super::{hit_record::{HitRecord}, hittable::TrHittable};

pub struct HittableList {
    pub objects: Vec<Box<dyn TrHittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn TrHittable>>) -> Self {
        Self { objects }
    }
    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Box<dyn TrHittable>) {
        self.objects.push(object)
    }
}

impl HittableList {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::empty();
        let mut hit_anything = false;
        let mut closesy_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closesy_so_far, &mut temp_rec) {
                hit_anything = true;
                hit_record.copy(&temp_rec);
                closesy_so_far = hit_record.t;
            }
        }
        return hit_anything;
    }
}
