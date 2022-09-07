use crate::{ray_tracing::ray::Ray, math_f64::vec3::Color};

use super::hit_record::HitRecord;



pub trait TrHittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
