use crate::math_f64::vec3::Vec3;

use super::ray::Ray;

pub struct Camera {
    pub width: f64,
    pub height: f64,
    pub aspect_retio: f64,
    pub focal_lehgth: f64,
    pub origin: Vec3,

    pub lower_left_corner: Vec3,
}

impl Camera {
  pub fn get_ray(&self, u:f64,v:f64)->Ray{
        Ray ::from_camera(self, Vec3 ::new([u,v,0.0]))
    }
}
///Constructor
impl Camera {
    pub fn new(
        height: f64,
        aspect_retio: f64,
        focal_lehgth: Option<f64>,
        origin: Option<Vec3>,
    ) -> Camera {
        let width = aspect_retio * height;
        let horizontal = Vec3::new([width, 0.0, 0.0]);
        let vertical = Vec3::new([0.0, height, 0.0]);

        let orig = match origin {
            None => Vec3::new([0.0, 0.0, 0.0]),
            Some(vec) => vec,
        };

        let focal_lehgth = match focal_lehgth {
            None => 1.0,
            Some(f) => f,
        };

        let lower_left_corner =
            orig - horizontal / 2.0 - vertical / 2.0 - Vec3::new([0.0, 0.0, focal_lehgth]);

        Camera {
            width,
            height,
            aspect_retio,
            focal_lehgth,
            origin: orig,

            lower_left_corner,
        }
    }
}
