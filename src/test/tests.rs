use crate::math_f64::vec3::{Point3, Vec3};
use crate::model::hit_record::{HitRecord};
use crate::model::sphere::Sphere;
use crate::model::hittable::TrHittable;
use crate::ray_tracing::ray::Ray;

pub fn test_record() {
   // let sphere = Sphere::new(Point3::new([0.0, 0.0, -1.0]), 0.5);

    let ray = Ray::new(Vec3::new([0.0; 3]), Point3::new([0.0, 0.0, -1.0]));
    let mut rec = HitRecord::empty();
 //   sphere.hit(&ray, 0.5, 0.5, &mut rec);
}
