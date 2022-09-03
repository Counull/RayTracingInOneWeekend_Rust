

use crate::utils::file_utils::{self, file_write};



pub struct Image {
    pub width: i32,
    pub height: i32,
}

impl Image {
    pub fn new(width: i32, height: i32) -> Self { Self { width, height } }

   pub fn from_width(width: i32, aspect_retio: f64) -> Image {
        Image {
            width,
            height: (width as f64 / aspect_retio) as i32,
        }
    }
}

impl Image {
    pub fn generate_image(&self,path:&str,rgb_str:String) {
        let mut ret = String::new();
        ret.push_str(&format!("P3\n{} {}\n255\n", self.width, self.height));
        ret.push_str(&rgb_str);
        file_write(path,ret.as_bytes());
    }
}