use crate::{
    common::math::Interval,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

/// Models our little raytracing world - which is just a list of Hittable objects
#[derive(Default)]
pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut temp_record = None;
        let mut closest_so_far = interval.max();

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, Interval::new(interval.min(), closest_so_far)) {
                closest_so_far = rec.t;
                temp_record = Some(rec);
            }
        }

        temp_record
    }
}
