use std::{f32::INFINITY, io::Write};
mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use color::{write_color, Color};
use core::panic;
use hittable::{HitRecord, Hittable};
use ray::Ray;
use sphere::Sphere;
use std::fs::OpenOptions;
use vec3::{Point3, Vec3};

/**
 * IMAGE
 */
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

/**
 * CAMERA
 */
const VIEWPORT_HEIGHT: f64 = 2.0;
// Not using ASPECT_RATIO directly here since it may not be the _actual_ ratio between the
// the image dimensions given that they are not real-valued.
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
const FOCAL_LENGTH: f64 = 1.0;

fn ray_color<T: Hittable>(ray: Ray, obj: &T) -> Color {
    let mut rec: HitRecord = HitRecord::new();
    let did_hit = obj.hit(&ray, 0.0, INFINITY.into(), &mut rec);
    if did_hit {
        0.5 * rec.normal.map(|x| -> f64 { x + 1.0 })
    } else {
        let unit_direction = ray.direction().into_unit();
        let t = 0.5 * (unit_direction.1 + 1.0);
        (1.0 - t) * Color::from(1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    if IMAGE_HEIGHT < 1 {
        panic!("IMAGE_HEIGHT is way too small, use a larger width");
    }

    println!("Image dimensions: {} ✕ {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!(
        "Viewport dimensions: {} ✕ {}",
        VIEWPORT_WIDTH, VIEWPORT_HEIGHT
    );

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) // Clear contents
        .create(true)
        .open("./image.ppm")
        .unwrap();

    let camera_center = Point3::from(0.0);

    let viewport_horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_vertical = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);
    let viewport_upper_left = camera_center
        - Vec3::new(0.0, 0.0, FOCAL_LENGTH)
        - viewport_horizontal / 2.0
        - viewport_vertical / 2.0;

    let pixel_delta_u = viewport_horizontal / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_vertical / IMAGE_HEIGHT as f64;
    let pixel_upper_left = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    writeln!(file, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).expect("writing header");

    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel_upper_left + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray = Ray::new(camera_center, pixel_center - camera_center);
            let pixel_color = ray_color(ray, &sphere);
            write_color(&mut file, pixel_color);
        }
    }
}
