use crate::hittable::{HitRecord, Hittable};

// Our world is just a list of Hittable objects
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
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        record: &mut crate::hittable::HitRecord,
    ) -> bool {
        let mut temp_record = HitRecord::new();
        let mut did_hit_something = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_record) {
                did_hit_something = true;
                closest_so_far = temp_record.t;
                *record = temp_record.clone();
            }
        }

        did_hit_something
    }
}
