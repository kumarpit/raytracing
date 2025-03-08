use crate::{hittable::Hittable, vec3::Point3};

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
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        record: &mut crate::hittable::HitRecord,
    ) -> bool {
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
        if root <= t_min || root >= t_max {
            root = (h + sqrt_d) / a;
            if root <= t_min || root >= t_max {
                return false;
            }
        }

        record.t = root;
        record.point = ray.at(root);
        record.normal = (record.point - self.center) / self.radius; // Normalize

        true
    }
}
