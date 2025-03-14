use std::io::Write;

use crate::{
    color::{write_color, Color},
    common::math::{lerp, random, Interval, INFINITY},
    config::CameraConfig,
    hittable::Hittable,
    ray::Ray,
    vec3::{Point3, Vec3},
    world::World,
};

struct ImageProperties {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
}

impl ImageProperties {
    fn new(config: &CameraConfig) -> Self {
        let aspect_ratio = config.aspect_ratio[0] / config.aspect_ratio[1];
        let image_height = ((config.image_width as f64) / aspect_ratio) as i32;
        ImageProperties {
            aspect_ratio,
            image_width: config.image_width,
            image_height,
        }
    }
}

struct ViewportProperties {
    viewport_width: f64,
    viewport_height: f64,
    pixel_upper_left: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl ViewportProperties {
    fn new(config: &CameraConfig, image_properties: &ImageProperties) -> Self {
        let viewport_height = config.viewport_height;
        // Not using ASPECT_RATIO directly here since it may not be the _actual_ ratio between the
        // the image dimensions given that they are not real-valued.
        let viewport_width = viewport_height
            * (image_properties.image_width as f64 / image_properties.image_height as f64);
        let center = Point3::from(0.0);
        let viewport_horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_vertical = Vec3::new(0.0, -viewport_height, 0.0);
        let viewport_upper_left = center
            - Vec3::new(0.0, 0.0, config.focal_length)
            - viewport_horizontal / 2.0
            - viewport_vertical / 2.0;

        let pixel_delta_u = viewport_horizontal / config.image_width as f64;
        let pixel_delta_v = viewport_vertical / image_properties.image_height as f64;
        let pixel_upper_left = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        ViewportProperties {
            viewport_width,
            viewport_height,
            pixel_upper_left,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
}

pub struct Camera {
    center: Point3,
    focal_length: f64,
    samples_per_pixel: i32,
    max_ray_bounces: i32,
    image_properties: ImageProperties,
    viewport_properties: ViewportProperties,
}

impl Camera {
    pub fn new(config: &CameraConfig) -> Self {
        let image_properties = ImageProperties::new(config);
        let viewport_properties = ViewportProperties::new(config, &image_properties);
        Camera {
            center: Point3::from(0.0),
            focal_length: config.focal_length,
            samples_per_pixel: config.samples_per_pixel,
            max_ray_bounces: config.max_ray_bounces,
            image_properties,
            viewport_properties,
        }
    }

    pub fn render(&self, world: &World, out: &mut impl Write) {
        if self.image_properties.image_height < 1 {
            panic!("IMAGE_HEIGHT is way too small, use a larger width");
        }

        println!(
            "Image dimensions: {} ✕ {}",
            self.image_properties.image_width, self.image_properties.image_height
        );
        println!(
            "Viewport dimensions: {} ✕ {}",
            self.viewport_properties.viewport_width, self.viewport_properties.viewport_height
        );

        writeln!(
            out,
            "P3\n{} {}\n255\n",
            self.image_properties.image_width, self.image_properties.image_height
        )
        .expect("writing header");
        for j in 0..self.image_properties.image_height {
            eprint!(
                "\rRendering progress: {} / {}",
                j, self.image_properties.image_height
            );
            for i in 0..self.image_properties.image_width {
                let mut pixel_color = Color::from(0.0);
                // Anti-aliasing
                (0..self.samples_per_pixel).for_each(|_| {
                    let ray = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(&ray, world, self.max_ray_bounces);
                });
                write_color(out, pixel_color / self.samples_per_pixel as f64);
            }
        }
    }

    fn ray_color<T: Hittable>(&self, ray: &Ray, obj: &T, depth: i32) -> Color {
        if depth <= 0 {
            return Color::from(0.0);
        }

        // Having the interval start at 0.001 helps resolve "shadow acne"
        if let Some(rec) = obj.hit(&ray, Interval::new(0.001, INFINITY)) {
            let mut attenuation = Color::default();
            let mut scattered = Ray::default();
            if rec
                .material
                .scatter(ray, &rec, &mut attenuation, &mut scattered)
            {
                attenuation * self.ray_color(&scattered, obj, depth - 1)
            } else {
                Color::from(0.0)
            }
        } else {
            // Generates a blue-to-white gradient background
            let unit_direction = ray.direction().into_unit();
            let t = 0.5 * (unit_direction.1 + 1.0);
            lerp(Color::from(1.0), Color::new(0.5, 0.7, 1.0), t)
        }
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Vec3(random() - 0.5, random() - 0.5, 0.0);
        let pixel_sample = self.viewport_properties.pixel_upper_left
            + ((i as f64 + offset.0) * self.viewport_properties.pixel_delta_u)
            + ((j as f64 + offset.1) * self.viewport_properties.pixel_delta_v);
        Ray::new(self.center, pixel_sample - self.center)
    }
}
