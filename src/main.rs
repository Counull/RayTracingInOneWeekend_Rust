mod math_f64;
mod ray_tracing;
mod utils;
use math_f64::vec3::Color;


use ray_tracing::{camera::Camera, image::Image};
use utils::file_utils::file_write;

fn main() {
    let camera = Camera::new(2.0, 16.0 / 9.0, None, None);
    let image = Image::from_width(400, camera.aspect_retio);
   

    let mut rgb_str = String::new();

    let mut j = image.height - 1;

    while j >= 0 {
        let mut i = 0;
        while i < image.width {
            let color = Color::from_array([
                i as f64 / (image.width - 1) as f64,
                j as f64 / (image.height - 1) as f64,
                0.25,
            ]);

            rgb_str.push_str(&format!(
                "{} {} {}\n",
                (255.999 * color.x()) as i32,
                (255.999 * color.y()) as i32,
                (255.999 * color.z()) as i32
            ));
            i += 1
        }
        j -= 1;
    }
   image.generate_image(rgb_str);
}
