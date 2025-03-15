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

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_bubble = Rc::new(Dielectric::new(1.00 / 1.50));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let mut world = World::new();

    // TODO: would be nice to have some sort of a DSL to describe the world -- Rust macros!
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let camera = Camera::new(&camera_config);
    camera.render(&world, &mut file);
}
