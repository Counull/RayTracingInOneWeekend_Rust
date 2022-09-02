use crate::math_f64::vec3::{Point3, Vec3};

use super::camera::{self, Camera};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

///Constructor
impl Ray {
    pub fn new() -> Ray {
        Ray {
            orig: Point3::new(),
            dir: Vec3::new(),
        }
    }

    pub fn from_vec3(orig: Point3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }

    pub fn from_array(origin: [f64; 3], direct: [f64; 3]) -> Ray {
        Ray {
            orig: Point3::from_array(origin),
            dir: Vec3::from_array(direct),
        }
    }

    ///Generate ray from camera to screen coordinate
    ///program screen_coord: just use .x() and .y()
    pub fn from_camera(camera: &Camera, screen_coord: Vec3) -> Ray {
        Ray {
            orig: camera.origin,
            dir: Vec3::unit_vector(
                camera.lower_left_corner
                    + Vec3::from_array([
                        screen_coord.x() * camera.width,
                        screen_coord.y() * camera.height,
                        0.0,
                    ])
                    - camera.origin,
            ),
        }
    }
}

impl Ray {
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direct(&self) -> Vec3 {
        self.dir
    }
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        return self.orig + t * self.dir;
    }
}
