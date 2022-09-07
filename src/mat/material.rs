use crate::{ray_tracing::ray::Ray, model::hit_record::HitRecord, math_f64::vec3::Color};

pub  trait TrMaterial {
   fn scatter(& self,ray: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}
