mod test;
mod math_f64;
mod ray_tracing;
mod utils;

use crate::math_f64::vec3::{Color, Point3, Vec3};
use ray_tracing::{camera::Camera, image::Image, ray::Ray};


fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(&Point3::new([0.0, 0.0, -1.0]), 0.5, &r);

    if t > 0.0 {
        let N = Vec3::unit_vector(r.at(t) - Vec3::new([0.0, 0.0, -1.0]));
        return 0.5 * Color::new([N.x() + 1.0, N.y() + 1.0, N.z() + 1.0]);
    }

    let t = 0.5 * (r.dir.y() + 1.0);
    (1.0 - t) * Color::new([1.0, 1.0, 1.0]) + t * Color::new([0.5, 0.7, 1.0])
}

//实际上就是求一元二次方程是否有解
fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - *center; //预先计算（A-C）

    //b^2-4ac
    let a = ray.dir.length_squared();
    let half_b = Vec3::dot(ray.dir, oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c; //b^2-4ac

    if discriminant < 0.0 {
        return -1.0;
    }

    return (-half_b - discriminant.sqrt()) / a;
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
            let r = Ray::from_camera(&camera, Vec3::new([u, v, 0.0]));
            let pix_color = ray_color(r);
            rgb_str.push_str(&pix_color.to_r8g8b8_string());
            i += 1
        }
        j -= 1;
    }
    image.generate_image("../image/result.ppm", rgb_str);
}
