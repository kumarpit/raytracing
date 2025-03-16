mod camera;
mod color;
mod common;
mod config;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;
mod world;

use camera::Camera;
use color::Color;
use common::math::{random, random_in_range};
use material::Material;
use sphere::Sphere;
use std::{fs::OpenOptions, sync::Arc};
use vec3::Point3;
use world::World;

fn main() {
    let config = config::Config::new();
    let camera_config = config.camera.unwrap();
    let out_config = config.out.unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) // Clear contents
        .create(true)
        .open(out_config.file)
        .unwrap();

    let mut world = World::new();

    let ground_material = Arc::new(Material::Lambertian {
        albedo: Color::from(0.5),
    });
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random();
            let center = Point3::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    // Diffuse material
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Material::Lambertian { albedo });
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_material < 0.95 {
                    // Metal material
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = random_in_range(0.0, 0.5);
                    let sphere_material = Arc::new(Material::Metal { albedo, fuzz });
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Material::Dielectric {
                        refractive_index: 1.5,
                    });
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material_1 = Arc::new(Material::Dielectric {
        refractive_index: 1.5,
    });
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Arc::new(Material::Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Arc::new(Material::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    let camera = Camera::new(&camera_config);
    camera.render(&world, &mut file);
}
