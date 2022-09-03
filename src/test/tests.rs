

use crate::ray_tracing::hittable::{Hittable, HitRecord};
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::sphere::Sphere;
use crate::math_f64::vec3::{Vec3, Point3};



pub fn test_record() {
let sphere = Sphere::new(Point3::new([0.0, 0.0, -1.0]), 0.5);

let ray = Ray::new(Vec3::new([0.0;3]) , Point3::new([0.0, 0.0, -1.0]));
let mut rec = HitRecord::empty();
sphere.hit(&ray, 0.5, 0.5, & mut rec);

}