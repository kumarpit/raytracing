use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

/// We follow the convention that the `normal` always points in the opposite direction
/// of the ray. This is purely for efficiency, since we have more material types than
/// geometry types - thus it is less work to determine which face the ray hit during
/// geometry intersection.
#[derive(Clone, Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub did_hit_front_frace: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Default::default()
    }

    /// Assumes outward_normal is of unit length
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.did_hit_front_frace = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.did_hit_front_frace {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval, record: &mut HitRecord) -> bool;
}
