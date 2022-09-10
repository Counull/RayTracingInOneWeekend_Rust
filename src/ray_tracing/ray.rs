use super::camera::Camera;
use crate::math_f64::vec3::{Point3, Vec3};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

///Constructor
impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self {
            orig: orig,
            dir: dir,
        }
    }

    pub fn empty() -> Self {
        Self {
            orig: Point3::empty(),
            dir: Vec3::empty(),
        }
    }

    pub fn from_array(origin: [f64; 3], direct: [f64; 3]) -> Ray {
        Ray {
            orig: Point3::new(origin),
            dir: Vec3::new(direct),
        }
    }

    ///Generate ray from camera to screen coordinate
    ///program screen_coord: just use .x() and .y()
    ///这个函数是否需要存在？因为这里所以需要的变量其实多数属于Camera，这个函数的归属权应是Camera
    pub fn from_camera(camera: &Camera, screen_coord: Vec3) -> Ray {
        Ray {
            orig: camera.origin,
            dir: camera.lower_left_corner
                + screen_coord.x() * camera.horizontal
                + screen_coord.y() * camera.vertical
                - camera.origin,
        }
    }
}

impl Ray {
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        return self.orig + t * self.dir;
    }
}
