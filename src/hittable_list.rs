use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable, HittableObject},
    interval::Interval,
    ray::Ray,
};

#[derive(Debug, Default)]
pub struct HittableList {
    objects: Vec<HittableObject>,
}

impl HittableList {
    pub fn new(object: HittableObject) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: HittableObject) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_rec: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.get_max();

        for object in &self.objects {
            if let Some(rec) = object.hit(r, Interval::new(ray_t.get_min(), closest_so_far)) {
                closest_rec = Some(rec);
                closest_so_far = rec.t;
            }
        }

        closest_rec
    }
}

impl Into<HittableObject> for HittableList {
    fn into(self) -> HittableObject {
        Rc::new(self)
    }
}
