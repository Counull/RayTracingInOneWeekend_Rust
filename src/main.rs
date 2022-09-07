mod mat;
mod math_f64;
mod model;
mod ray_tracing;
mod test;
mod utils;
use std::rc::Rc;

use crate::math_f64::{
    mathf64::{random_f64, random_f64_01},
    vec3::{Color, Point3, Vec3},
};
use mat::lambertian::Lambertian;
use math_f64::mathf64::{self};
use model::{hit_record::HitRecord, hittable_list::HittableList, sphere::Sphere};
use ray_tracing::{camera::Camera, image::Image, ray::Ray};

fn ray_color(r: Ray, world: &HittableList, depth: i32) -> Color {
    let mut rec = HitRecord::empty();
    if depth <= 0 {
        return Color::empty();
    }

    if world.hit(&r, 0.001, mathf64::infinity, &mut rec) {
        match &rec.mat {
            Some(m) => {
                let mut attenuation = Color::empty();
                let mut scattered = Ray::empty();
                m.scatter(&r, &rec, &mut attenuation, &mut scattered);
                return attenuation * ray_color(scattered, world, depth - 1);
            }
            None => {
                          print!("None mat Error");
            }
        }

        //   let target = rec.p + Vec3::random_in_hemisphere(rec.normal);
        //在碰撞点上建立一个球体之后取随机方向，同时使这个随机值和法线在球体同一侧
    }

    let unit_dir = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Color::new([1.0, 1.0, 1.0]) + t * Color::new([0.5, 0.7, 1.0])
}

fn main() {
    let camera = Camera::new(2.0, 16.0 / 9.0, None, None);
    let image = Image::from_width(400, camera.aspect_retio);

    let mut world = HittableList::empty();

    let v1 = &Vec3::new([1.0, 1.0, 1.0]);
    let v2 = *v1;

    let lambertian_mat = Rc::new(Lambertian::new(Vec3::new([0.9, 0.0, 0.0])));
    let lambertian_mat1 = Rc::new(Lambertian::new(Vec3::new([0.9, 0.9, 0.9])));

    world.add(Box::new(Sphere::new(
        Point3::new([0.0, 0.0, -1.0]),
        0.5,
        lambertian_mat.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new([0.0, -100.5, -1.0]),
        100.0,
        lambertian_mat1.clone(),
    )));

    let sample_per_pixel = 100;
    let max_depth = 50;
    let mut rgb_str = String::new();
    let scale = 1.0 / sample_per_pixel as f64;
    let mut j = image.height - 1;
    while j >= 0 {
        println!("Scanlines remaining:{}", j);
        let mut i = 0;
        while i < image.width {
            let mut pix_color = Vec3::empty();
            let mut s = 0;
            while s < sample_per_pixel {
                let u = (i as f64 + random_f64_01()) / ((image.width - 1) as f64);
                let v = (j as f64 + random_f64_01()) / ((image.height - 1) as f64);

                let r = camera.get_ray(u, v);
                pix_color += ray_color(r, &world, max_depth);
                s += 1;
            }

            pix_color *= scale;
            rgb_str.push_str(&pix_color.to_r8g8b8_string());
            i += 1
        }
        j -= 1;
    }
    image.generate_image("image/result.ppm", rgb_str);
}
