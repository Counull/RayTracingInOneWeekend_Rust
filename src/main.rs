mod mat;
mod math_f64;
mod model;
mod ray_tracing;
mod test;
mod utils;
use crate::math_f64::{
    mathf64::{random_f64, random_f64_01},
    vec3::{Color, Point3, Vec3},
};
use mat::{dielectric::Dielectirc, lambertian::Lambertian, metal::Metal};
use math_f64::mathf64::{self, PI};
use model::{
    hit_record::HitRecord,
    hittable_list::{self, HittableList},
    sphere::Sphere,
};
use ray_tracing::{
    camera::{self, Camera},
    image::Image,
    ray::Ray,
};
use std::rc::Rc;
use std::time::{Duration, SystemTime};
fn random_scence() -> HittableList {
    let mut world = HittableList::empty();

    let lambertian_ground = Rc::new(Lambertian::new(Vec3::new([0.5, 0.5, 0.5])));

    world.add(Box::new(Sphere::new(
        Point3::new([0., -1000., 0.]),
        1000.,
        lambertian_ground.clone(),
    ))); //ground

    //Big Ball
    let glass_mat = Rc::new(Dielectirc::new(1.5));
    let diffuse_mat = Rc::new(Lambertian::new(Vec3::new([0.4, 0.2, 0.1])));
    let metal_mat = Rc::new(Metal::new(Vec3::new([0.7, 0.6, 0.5]), 0.));
    world.add(Box::new(Sphere::new(
        Point3::new([0., 1., 0.]),
        1.,
        glass_mat,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new([-4., 1., 0.]),
        1.,
        diffuse_mat,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new([4., 1., 0.]),
        1.,
        metal_mat,
    )));

    for a in -11..10 {
        for b in -11..10 {
            let center = Point3::new([
                a as f64 + 0.9 * random_f64_01(),
                0.2,
                b as f64 + 0.9 * random_f64_01(),
            ]);
            if (center - Point3::new([4., 0.2, 0.])).length() > 0.9 {
                let choose_mat = random_f64_01();
                if choose_mat < 0.70 {
                    //diffuse
                    let albedo = Color::random() * Color::random();
                    let mat = Rc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, mat)));
                } else if choose_mat < 0.87 {
                    //metal
                    let albedo = Color::random_min_max(0.5, 1.);
                    let fuzz = random_f64(0., 0.5);
                    let mat = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, mat)));
                } else {
                    //glass
                    let mat = Rc::new(Dielectirc::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, mat)));
                }
            }
        }
    }

    return world;
}

fn ray_color(r: Ray, world: &HittableList, depth: i32) -> Color {
    let mut rec = HitRecord::empty();
    if depth <= 0 {
        return Color::empty();
    }

    if world.hit(&r, 0.001, mathf64::INFINITY, &mut rec) {
        match &rec.mat {
            Some(m) => {
                let mut attenuation = Color::empty();
                let mut scattered = Ray::empty();
                if m.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                    return attenuation * ray_color(scattered, world, depth - 1);
                }
                return Color::empty();
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

   

    let sy_time = SystemTime::now();
    let world = random_scence();
    println!(
        "Generate scenct done:{:?}",
        SystemTime::now()
            .duration_since(sy_time)
            .unwrap()
            .as_millis()
    );

    // let camera = Camera::new(2.0, 16.0 / 9.0, None, None);
    let lookfrom = Point3::new([13., 2., 3.]);
    let lookat = Point3::new([0., 0., 0.]);
    let camera = Camera::new(
        20.0,
        3.0 / 2.0,
        0.1,
        10.0,
        Some(lookfrom),
        Some(lookat),
        None,
    );
    let image = Image::from_width(1200, camera.aspect_retio);

    let sample_per_pixel = 510;
    let max_depth = 55;
    let mut rgb_str = String::new();
    let scale = 1.0 / sample_per_pixel as f64;

    let mut j = image.height - 1;
    while j >= 0 {
        println!(
            "Scanlines remaining:{} ,{:?}",
            j,
            SystemTime::now()
                .duration_since(sy_time)
                .unwrap()
                .as_secs_f64()
        );
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
    println!(
        "Ray trace done:{:?}",
        SystemTime::now()
            .duration_since(sy_time)
            .unwrap()
            .as_secs_f64()
    );

    image.generate_image("image/result.ppm", rgb_str);
    println!(
        "Generate image done:{:?}",
        SystemTime::now()
            .duration_since(sy_time)
            .unwrap()
            .as_secs_f64()
    );
}
