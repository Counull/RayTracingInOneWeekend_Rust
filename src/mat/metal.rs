use crate::{
    math_f64::vec3::{Color, Vec3},
    ray_tracing::ray::Ray,
};

use super::material::TrMaterial;

pub struct Metal {
    albedo: Color,
}

impl TrMaterial for Metal {
    fn scatter(
        &self,
        ray: &crate::ray_tracing::ray::Ray,
        rec: &crate::model::hit_record::HitRecord,
        attenuation: &mut Color,
        scattered: &mut crate::ray_tracing::ray::Ray,
    ) -> bool {
        let reflected = Vec3::unit_vector(ray.direction()).reflect(&rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        Vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}
