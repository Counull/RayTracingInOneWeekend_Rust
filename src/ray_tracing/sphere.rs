use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use crate::math_f64::vec3::{Point3, Vec3};

pub struct Sphere {
    pub cneter: Point3,
    pub radius: f64,
}

///Constructor
impl Sphere {
    pub fn new(cneter: Point3, radius: f64) -> Self {
        Self { cneter, radius }
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

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64,   rec:  &mut HitRecord) -> bool {
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
        let outward_normal = (rec.p - self.cneter) / self.radius;
    
        rec.record_hit(root, r, outward_normal);

        return true;
    }
}
