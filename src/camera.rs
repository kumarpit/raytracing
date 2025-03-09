use std::io::Write;

use crate::{
    color::{write_color, Color},
    common::{lerp, random, INFINITY},
    hittable::{HitRecord, Hittable},
    interval::{Interval, ANTI_SHADOW_ACNE_HIT_INTERVAL, EMPTY_INTERVAL, POSITIVE_INTERVAL},
    ray::Ray,
    vec3::{Point3, Vec3},
    world::World,
};

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

pub struct Camera {
    aspect_ratio: f64,
    viewport_height: f64,
    viewport_width: f64,
    image_width: i32,
    image_height: i32,
    center: Point3,
    focal_length: f64,
    pixel_upper_left: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i32,
    max_depth: i32,
}

impl Camera {
    pub fn new() -> Self {
        let center = Point3::from(0.0);
        let viewport_horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let viewport_vertical = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);
        let viewport_upper_left = center
            - Vec3::new(0.0, 0.0, FOCAL_LENGTH)
            - viewport_horizontal / 2.0
            - viewport_vertical / 2.0;

        let pixel_delta_u = viewport_horizontal / IMAGE_WIDTH as f64;
        let pixel_delta_v = viewport_vertical / IMAGE_HEIGHT as f64;
        let pixel_upper_left = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio: ASPECT_RATIO,
            viewport_height: VIEWPORT_HEIGHT,
            viewport_width: VIEWPORT_WIDTH,
            image_width: IMAGE_WIDTH,
            image_height: IMAGE_HEIGHT,
            center,
            focal_length: FOCAL_LENGTH,
            pixel_upper_left,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel: 100,
            max_depth: 100,
        }
    }

    pub fn render(&self, world: &World, out: &mut impl Write) {
        if self.image_height < 1 {
            panic!("IMAGE_HEIGHT is way too small, use a larger width");
        }

        println!("Image dimensions: {} ✕ {}", IMAGE_WIDTH, IMAGE_HEIGHT);
        println!(
            "Viewport dimensions: {} ✕ {}",
            VIEWPORT_WIDTH, VIEWPORT_HEIGHT
        );

        writeln!(out, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).expect("writing header");
        for j in 0..IMAGE_HEIGHT {
            eprint!("\rRendering progress: {} / {}", j, IMAGE_HEIGHT);
            for i in 0..IMAGE_WIDTH {
                let mut pixel_color = Color::from(0.0);
                // Anti-aliasing
                (0..self.samples_per_pixel).for_each(|_| {
                    let ray = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(&ray, world, self.max_depth);
                });
                write_color(out, pixel_color / self.samples_per_pixel as f64);
            }
        }
    }

    fn ray_color<T: Hittable>(&self, ray: &Ray, obj: &T, depth: i32) -> Color {
        let mut rec: HitRecord = HitRecord::new();

        if depth <= 0 {
            return Color::from(0.0);
        }

        if obj.hit(&ray, ANTI_SHADOW_ACNE_HIT_INTERVAL, &mut rec) {
            let direction = Vec3::random_on_hemisphere(rec.normal);
            0.5 * self.ray_color(&Ray::new(rec.point, direction), obj, depth - 1)
        } else {
            // Generates a blue-to-white gradient background
            let unit_direction = ray.direction().into_unit();
            let t = 0.5 * (unit_direction.1 + 1.0);
            lerp(Color::from(1.0), Color::new(0.5, 0.7, 1.0), t)
        }
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Vec3(random() - 0.5, random() - 0.5, 0.0);
        let pixel_sample = self.pixel_upper_left
            + ((i as f64 + offset.0) * self.pixel_delta_u)
            + ((j as f64 + offset.1) * self.pixel_delta_v);
        Ray::new(self.center, pixel_sample - self.center)
    }
}
