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
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use std::{fs::OpenOptions, rc::Rc};
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

    let ground_material = Rc::new(Lambertian::new(Color::from(0.5)));
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
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_material < 0.95 {
                    // Metal material
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = random_in_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material_1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    let camera = Camera::new(&camera_config);
    camera.render(&world, &mut file);
}
