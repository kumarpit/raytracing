mod camera;
mod color;
mod common;
mod hittable;
mod ray;
mod sphere;
mod vec3;
mod world;

use camera::Camera;
use sphere::Sphere;
use std::fs::OpenOptions;
use vec3::Point3;
use world::World;

fn main() {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true) // Clear contents
        .create(true)
        .open("./image.ppm")
        .unwrap();

    // Initialize world
    let mut world = World::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();
    camera.render(world, file);
}
