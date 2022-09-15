use crate::{math_f64::vec3::{Color, Vec3}, ray_tracing::ray::Ray, model::hit_record::HitRecord};

use super::material::TrMaterial;


pub struct Lambertian{
albedo:Color
}

impl TrMaterial for Lambertian {
    fn scatter( & self,ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction=rec.normal+Vec3::random_unit_vector();
    
    if scatter_direction.near_zero() {
        scatter_direction = rec.normal;
    }
    
        * scattered =  Ray::new(rec.p, scatter_direction,Some(ray_in.time));
      * attenuation =  self.albedo;
     
            return true;
    } 
}

impl Lambertian {
  pub  fn new(albedo: Color) -> Self { Self { albedo } }
}