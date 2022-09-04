mod math_f64;
mod model;
mod ray_tracing;
mod test;
mod utils;

use crate::math_f64::vec3::{Color, Point3, Vec3};
use math_f64::mathf64;
use model::{hittable_list::HittableList, sphere::Sphere, hittable::HitRecord};
use ray_tracing::{camera::Camera, image::Image, ray::Ray};

fn ray_color(r: Ray,world:&  HittableList) -> Color {

    let mut rec = HitRecord::empty();
    if world.hit(&r, 0.0,mathf64::infinity ,  &mut rec) {
    
        return 0.5 * (rec.normal + Color::new([1.0,1.0,1.0]));
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

    let mut world = HittableList::empty();
    world.add(Box::new(Sphere::new(Point3::new([0.0, 0.0, -1.0]), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new([0.0, -100.5,-1.0]), 100.0)));
   
    let mut rgb_str = String::new();

    let mut j = image.height - 1;
    while j >= 0 {
        println!("Scanlines remaining:{}", j);
        let mut i = 0;
        while i < image.width {
            let u = (i as f64) / ((image.width - 1) as f64);
            let v = (j as f64) / ((image.height - 1) as f64);
            let r = Ray::from_camera(&camera, Vec3::new([u, v, 0.0]));
            let pix_color = ray_color(r,&mut world);
            rgb_str.push_str(&pix_color.to_r8g8b8_string());
            i += 1
        }
        j -= 1;
    }
    image.generate_image("../image/result.ppm", rgb_str);
}
