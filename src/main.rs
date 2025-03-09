mod camera;
mod color;
mod common;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;
mod world;

use camera::Camera;
use color::Color;
use material::{Lambertian, Metal};
use sphere::Sphere;
use std::{fs::OpenOptions, rc::Rc};
use vec3::Point3;
use world::World;

fn main() {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) // Clear contents
        .create(true)
        .open("./image.ppm")
        .unwrap();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::from(0.8)));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    let mut world = World::new();

    // TODO: would be nice to have some sort of a DSL to describe the world -- Rust macros!
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let camera = Camera::new();
    camera.render(&world, &mut file);
}
