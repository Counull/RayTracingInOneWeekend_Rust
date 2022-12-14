use std::borrow::Borrow;
use std::rc::Rc;

use super::hit_record::HitRecord;
use super::hittable::TrHittable;
use crate::mat::material::{self, TrMaterial};
use crate::math_f64::vec3::{Point3, Vec3};
use crate::ray_tracing::ray::Ray;

pub struct Sphere {
    pub radius: f64,
    pub cneter: Point3,
    pub mat: Rc<dyn TrMaterial>,
}

///Constructor
impl Sphere {
    pub fn new(cneter: Point3, radius: f64, mat: Rc<dyn TrMaterial>) -> Self {
        Self {
            cneter,
            radius,
            mat,
        }
    }

    //实际上就是求一元二次方程是否有解
    fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
        let oc = ray.origin() - *center; //预先计算（A-C）

        //b^2-4ac
        let a = ray.dir.length_squared();
        let half_b = Vec3::dot(ray.dir, oc);
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c; //b^2-4ac

        if discriminant < 0.0 {
            return -1.0;
        }

        return (-half_b - discriminant.sqrt()) / a;
    }
}

impl TrHittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.cneter; //预先计算（A-C）

        //b^2-4ac
        let a = r.dir.length_squared();
        let half_b = Vec3::dot(r.dir, oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c; //b^2-4ac

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        //求一元二次根
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        //  rec.record_hit(root, r, self);
        let hit_point = r.at(root);
        let outward_normal = (hit_point - self.cneter) / self.radius;

        rec.record_hit1(
            root,
            hit_point,
            r.dir,
            Some(self.mat.clone()),
            outward_normal,
        );

        return true;
    }
}
