use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval, record: &mut HitRecord) -> bool {
        // Ray-Sphere intersection
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - (a * c);

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies in the given range
        let mut root = (h - sqrt_d) / a;

        if !interval.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !interval.surrounds(root) {
                return false;
            }
        }

        record.t = root;
        record.point = ray.at(root);
        let outward_normal = (record.point - self.center) / self.radius; // Normalize
        record.set_face_normal(ray, outward_normal);

        true
    }
}
