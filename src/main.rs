mod utils;

mod vec3;
use utils::file_utils;
use vec3::Vec3;

use crate::{utils::file_utils::file_write, vec3::Color};

fn main() {
    let image_width = 256;
    let image_height = 256;
    let mut ret = String::new();

    ret.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));

    let mut j = image_height - 1;

    while j >= 0 {
        let mut i = 0;
        while i < image_width {
            let color = Color::from_array([
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            ]);

            ret.push_str(&format!(
                "{} {} {}\n",
                (255.999 * color.x()) as i32,
                (255.999 * color.y())as i32,
                (255.999 * color.z())as i32
            ));
            i += 1
        }
        j -= 1;
    }
    file_write(ret.as_bytes());
}
