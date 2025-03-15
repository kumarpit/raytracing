use std::sync::Arc;

use crate::{
    common::math::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

/// We follow the convention that the `normal` always points in the opposite direction
/// of the ray. This is purely for efficiency, since we have more material types than
/// geometry types - thus it is less work to determine which face the ray hit during
/// geometry intersection. Note that the normal should always be a unit vector.
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub did_hit_front_frace: bool,
}

impl HitRecord {
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

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}
