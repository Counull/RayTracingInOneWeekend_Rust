
use crate::math_f64::vec3::{Point3, Vec3};
use crate::ray_tracing::ray::Ray;

use super::sphere::Sphere;

pub  trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, front_face: bool) -> Self {
        Self {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn empty() -> Self {
        Self {
            p: Point3::empty(),
            normal: Vec3::empty(),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn copy( &mut self ,other:&Self){
      self.p=other.p;
      self.normal=other.normal;
      self.t=other.t;
      self.front_face=other.front_face;  
    }

  
}

///fn
impl HitRecord {
   
    pub fn set_face_normal(& mut self, r: &Ray, outward_normal: Vec3) {
    
        self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
          outward_normal
        } else {
            -outward_normal
        };
    }

    pub fn record_hit(& mut self, t: f64, r: &Ray, sphere: &Sphere) {
       
        self.t = t;
        self.p = r.at(self.t);
        let outward_normal = (self.p - sphere.cneter) / sphere.radius;
        self.set_face_normal(r, outward_normal);
    }
}
///refetor
impl HitRecord {
    pub fn record_hit1(& mut self, t: f64,  p :Vec3 ,ray_dir: Vec3,  outward_normal:Vec3) {
       
        self.t = t;
        self.p = p;
        self.set_face_normal1(ray_dir, outward_normal);
    }

    fn set_face_normal1(& mut self, ray_dir: Vec3, outward_normal: Vec3)  {
        self.front_face = Vec3::dot(ray_dir, outward_normal) < 0.0;
        self.normal = if self.front_face {
           
            outward_normal
        
        } else {
        
            -outward_normal
        };
    }
}
