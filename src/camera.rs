use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::io::Write;

use crate::{
    color::{write_color, Color},
    common::math::{deg_to_rad, lerp, random, Interval, INFINITY},
    config::CameraConfig,
    hittable::Hittable,
    ray::Ray,
    vec3::{Point3, Vec3},
    world::World,
};

struct ImageProperties {
    width: i32,
    height: i32,
}

impl ImageProperties {
    fn new(config: &CameraConfig) -> Self {
        let aspect_ratio = config.aspect_ratio[0] / config.aspect_ratio[1];
        let image_height = ((config.image_width as f64) / aspect_ratio) as i32;
        ImageProperties {
            width: config.image_width,
            height: image_height,
        }
    }
}

struct ViewportProperties {
    width: f64,
    height: f64,
    pixel_upper_left: Point3, // Position of the upper left pixel in the viewport (notice that it
    // is slightly inset from true top-left corner since we store the
    // center of each pixel)
    pixel_delta_u: Vec3, // Vector representing the distance between successive columns of pixels
    // in the viewport
    pixel_delta_v: Vec3, // Vector representing teh distance between successive rows of pixels in
                         // the viewport
}

impl ViewportProperties {
    fn new(config: &CameraConfig, image_properties: &ImageProperties) -> Self {
        // Determine the viewport dimensions
        let theta = deg_to_rad(config.vertical_field_of_view) / 2.0;
        let h = theta.tan();
        let viewport_height = 2.0 * h * config.focus_distance;
        // Not using ASPECT_RATIO directly here since it may not be the _actual_ ratio between the
        // the image dimensions given that they are not real-valued.
        let viewport_width =
            viewport_height * (image_properties.width as f64 / image_properties.height as f64);

        let (u, v, w) = Camera::get_basis_vectors(config);

        let center = Point3::from(config.lookfrom.clone());
        let viewport_horizontal = viewport_width * u; // vector across viewport horizontal
                                                      // edge
        let viewport_vertical = viewport_height * -v; // vector _down_ viewport vertical edge
        let viewport_upper_left = center
            - (config.focus_distance * w)
            - viewport_horizontal / 2.0
            - viewport_vertical / 2.0;

        let pixel_delta_u = viewport_horizontal / image_properties.width as f64;
        let pixel_delta_v = viewport_vertical / image_properties.height as f64;
        let pixel_upper_left = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        ViewportProperties {
            width: viewport_width,
            height: viewport_height,
            pixel_upper_left,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
}

pub struct Camera {
    center: Point3,
    defocus_angle: f64,
    defocus_disc_u: Vec3,
    defocus_disc_v: Vec3,
    samples_per_pixel: i32,
    max_ray_bounces: i32,
    image_properties: ImageProperties,
    viewport_properties: ViewportProperties,
}

impl Camera {
    pub fn new(config: &CameraConfig) -> Self {
        let image_properties = ImageProperties::new(config);
        let viewport_properties = ViewportProperties::new(config, &image_properties);

        let (u, v, _) = Camera::get_basis_vectors(config);

        // Calculate the camera defocus disc basis vectors
        let defocus_radius =
            config.focus_distance * deg_to_rad(config.defocus_angle as f64 / 2.0).tan();
        let defocus_disc_u = u * defocus_radius;
        let defocus_disc_v = v * defocus_radius;

        Camera {
            center: Point3::from(config.lookfrom.clone()),
            defocus_angle: config.defocus_angle as f64,
            defocus_disc_u,
            defocus_disc_v,
            samples_per_pixel: config.samples_per_pixel,
            max_ray_bounces: config.max_ray_bounces,
            image_properties,
            viewport_properties,
        }
    }

    pub fn render(&self, world: &World, out: &mut impl Write) {
        if self.image_properties.height < 1 {
            panic!("IMAGE_HEIGHT is way too small, use a larger width");
        }

        println!(
            "Image Dimensions: {} ✕ {}",
            self.image_properties.width, self.image_properties.height
        );
        println!(
            "Viewport Dimensions: {:.1} ✕ {:.1}",
            self.viewport_properties.width, self.viewport_properties.height
        );

        writeln!(
            out,
            "P3\n{} {}\n255\n",
            self.image_properties.width, self.image_properties.height
        )
        .expect("writing header");

        // More elegant progress bar than just eprintin'
        let bar = ProgressBar::new(self.image_properties.height as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{wide_bar}] {pos}/{len} rows")
                .unwrap()
                .progress_chars("#>-"),
        );
        bar.set_message("Rendering");

        for j in 0..self.image_properties.height {
            bar.inc(1);
            let pixel_colors: Vec<Color> = (0..self.image_properties.width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Color::from(0.0);
                    // Anti-aliasing
                    (0..self.samples_per_pixel).for_each(|_| {
                        let ray = self.get_ray(i, j);
                        pixel_color =
                            pixel_color + self.ray_color(&ray, world, self.max_ray_bounces);
                    });
                    pixel_color
                })
                .collect();
            for pixel_color in pixel_colors {
                write_color(out, pixel_color / self.samples_per_pixel as f64);
            }
        }
    }

    /// Computes the basis vectors for the camera's orientation
    fn get_basis_vectors(config: &CameraConfig) -> (Vec3, Vec3, Vec3) {
        let lookfrom = Point3::from(config.lookfrom.clone());
        let lookat = Point3::from(config.lookat.clone());
        let vup = Point3::from(config.vup.clone());

        let w = (lookfrom - lookat).into_unit();
        let u = vup.cross(&w).into_unit();
        let v = w.cross(&u);

        (u, v, w)
    }

    fn ray_color<T: Hittable>(&self, ray: &Ray, obj: &T, depth: i32) -> Color {
        if depth <= 0 {
            return Color::from(0.0);
        }

        // Having the interval start at 0.001 helps resolve "shadow acne"
        if let Some(rec) = obj.hit(&ray, Interval::new(0.001, INFINITY)) {
            rec.material
                .scatter(ray, &rec)
                .map(|scatter_record| {
                    scatter_record.attenuation
                        * self.ray_color(&scatter_record.scattered, obj, depth - 1)
                })
                .unwrap_or_else(|| Color::from(0.0))
        } else {
            // Generates a blue-to-white gradient background
            let unit_direction = ray.direction().into_unit();
            let t = 0.5 * (unit_direction.1 + 1.0);
            lerp(Color::from(1.0), Color::new(0.5, 0.7, 1.0), t)
        }
    }

    /// Constructs a ray originating from the defocus disc and directed at a randomly sampled point
    /// around the pixel location (i, j)
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Vec3(random() - 0.5, random() - 0.5, 0.0);
        let pixel_sample = self.viewport_properties.pixel_upper_left
            + ((i as f64 + offset.0) * self.viewport_properties.pixel_delta_u)
            + ((j as f64 + offset.1) * self.viewport_properties.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disc_sample()
        };
        Ray::new(ray_origin, pixel_sample - ray_origin)
    }

    /// Returns a random point in the camera defocus disc
    fn defocus_disc_sample(&self) -> Point3 {
        let p = Vec3::in_unit_disc();
        self.center + (p.0 * self.defocus_disc_u) + (p.1 * self.defocus_disc_v)
    }
}
