use crate::math_f64::{
    mathf64::{degrees_to_radians, random_f64},
    vec3::{Point3, Vec3},
};

use super::ray::Ray;

pub struct Camera {
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lookfrom: Vec3,
    pub lower_left_corner: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub aspect_retio: f64,
    pub lens_radius: f64,
    pub time0: f64,
    pub time1: f64,
}

///Constructor
/// prm:
/// focus_dist：模拟镜头景深，焦点与近平面的距离，当正好等于近平面与Origin的距离时画面完全聚焦
/// apertrue：光圈，聚焦平面大小，越大越模糊
impl Camera {
    pub fn new(
        vfov: f64,
        aspect_retio: f64,
        apertrue: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
        lookfrom: Option<Vec3>,
        lookat: Option<Point3>,
        vup: Option<Point3>,
    ) -> Camera {
        let lookfrom = match lookfrom {
            None => Vec3::new([0.0, 0.0, 0.0]),
            Some(vec) => vec,
        };

        let lookat = match lookat {
            None => Vec3::new([0.0, 0.0, -1.0]),
            Some(vec) => vec,
        };
        let vup = match vup {
            None => Vec3::new([0.0, 1.0, 0.0]),
            Some(vec) => vec,
        };

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;

        let viewport_width = aspect_retio * viewport_height;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        let lower_left_corner = lookfrom - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Camera {
            horizontal,
            vertical,
            lookfrom,
            lower_left_corner,
            time0,
            time1,
            u,
            v,
            w,
            aspect_retio,
            lens_radius: apertrue / 2.,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::from_camera(
            self,
            Vec3::new([u, v, 0.0]),
            Some(random_f64(self.time0, self.time1)),
        )
    }
}
