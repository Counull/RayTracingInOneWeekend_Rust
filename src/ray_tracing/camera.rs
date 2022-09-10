use crate::math_f64::{
    mathf64::degrees_to_radians,
    vec3::{Point3, Vec3},
};

use super::ray::Ray;

pub struct Camera {
    pub aspect_retio: f64,
    pub horizontal: Vec3,
    pub  vertical: Vec3,
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::from_camera(self, Vec3::new([u, v, 0.0]))
    }
}
///Constructor
impl Camera {
    pub fn new(
        height: f64,
        aspect_retio: f64,

        origin: Option<Vec3>,
        lookat: Option<Point3>,
        vup: Option<Point3>,
    ) -> Camera {
        let origin = match origin {
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

        let width = aspect_retio * height;

        let w = Vec3::unit_vector(origin - lookat);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let horizontal = width * u;
        let vertical = height * v;

        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            horizontal,
            vertical,
            aspect_retio,
            origin,
            lower_left_corner,
        }
    }
    pub fn from_fov(
        vfov: f64,
        aspect_retio: f64,

        origin: Option<Point3>,
        lookat: Option<Point3>,
        vup: Option<Point3>,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;

        Camera::new(viewport_height, aspect_retio, origin, lookat, vup)
    }
}
