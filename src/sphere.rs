use std::rc::Rc;

use crate::{
    common::math::Interval,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        // Ray-Sphere intersection
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - (a * c);

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies in the given range
        let mut root = (h - sqrt_d) / a;
        if !interval.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }

        // TODO: use a builder instead
        let mut record = HitRecord {
            t: root,
            point: ray.at(root),
            material: self.material.clone(),
            normal: Default::default(),
            did_hit_front_frace: Default::default(),
        };
        let outward_normal = (record.point - self.center) / self.radius; // Normalize
        record.set_face_normal(ray, outward_normal);

        Some(record)
    }
}
