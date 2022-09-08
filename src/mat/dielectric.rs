use crate::{
    math_f64::vec3::{Color, Vec3},
    ray_tracing::ray::Ray,
};

use super::material::TrMaterial;

///always refraction
pub struct Dielectirc {
    pub ir: f64, //refraction ratio
}

impl Dielectirc {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl TrMaterial for Dielectirc {
    fn scatter(
        &self,
        ray_in: &crate::ray_tracing::ray::Ray,
        rec: &crate::model::hit_record::HitRecord,
        attenuation: &mut crate::math_f64::vec3::Color,
        scattered: &mut crate::ray_tracing::ray::Ray,
    ) -> bool {
        *attenuation = Color::new([1.0, 1.0, 1.0]);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = Vec3::unit_vector(ray_in.direction());
        let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract {
           // print!("reflect");
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(&rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction);

        /*   let refracted = unit_direction.refract(&rec.normal, refraction_ratio);
         *scattered =  Ray::new(rec.p, refracted); */

        return true;
    }
}
