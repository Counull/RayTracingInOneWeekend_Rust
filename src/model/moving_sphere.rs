use std::rc::Rc;

use crate::{
    mat::material::TrMaterial,
    math_f64::vec3::{Point3, Vec3},
    ray_tracing::ray::Ray,
};

use super::{hit_record::HitRecord, hittable::TrHittable};

pub struct MoveSphere {
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub cneter0: Point3,
    pub cneter1: Point3,
    pub mat: Rc<dyn TrMaterial>,
}

impl MoveSphere {
    pub fn new(
        cneter0: Point3,
        cneter1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat: Rc<dyn TrMaterial>,
    ) -> Self {
        Self {
            time0,
            time1,
            radius,
            cneter0,
            cneter1,
            mat,
        }
    }
}
impl MoveSphere {
    pub fn center(&self, time: f64) -> Point3 {
        self.cneter0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.cneter1 - self.cneter0)
    }
}

impl TrHittable for MoveSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let current_center = self.center(r.time);
        let oc = r.origin() - current_center; //预先计算（A-C）

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
        let outward_normal = (hit_point - current_center) / self.radius;

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
