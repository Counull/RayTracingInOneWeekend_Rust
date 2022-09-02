mod math_f64;
mod ray_tracing;
mod utils;
use ray_tracing::{camera::Camera, image::Image, ray::Ray};
use utils::file_utils::file_write;

use math_f64::vec3::{Color, Vec3};

fn ray_color(r: Ray) -> Color {
    let t = 0.5 * (r.dir.y() + 1.0);
    (1.0 - t) * Color::from_array([1.0, 1.0, 1.0]) + t * Color::from_array([0.5, 0.7, 1.0])
}

fn main() {
    let camera = Camera::new(2.0, 16.0 / 9.0, None, None);
    let image = Image::from_width(400, camera.aspect_retio);

    let mut rgb_str = String::new();

    let mut j = image.height - 1;

    while j >= 0 {
        println!("Scanlines remaining:{}", j);
        let mut i = 0;
        while i < image.width {
            let u = (i as f64) / ((image.width - 1) as f64);
            let v = (j as f64) / ((image.height - 1) as f64);
            let r = Ray::from_camera(&camera, Vec3::from_array([u, v, 0.0]));
            let pix_color = ray_color(r);
            rgb_str.push_str(&pix_color.to_r8g8b8_string());
            i += 1
        }
        j -= 1;
    }
    image.generate_image(rgb_str);
}
