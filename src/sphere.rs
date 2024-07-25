use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable, HittableObject},
    interval::Interval,
    ray::Ray,
    vec3::Pos3,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    center: Pos3,
    radius: f64,
}

impl Sphere {
    pub const fn new(center: Pos3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = &self.center - r.origin();
        let a = r.direction().squared_abs();
        let h = r.direction().dot(&oc);
        let c = oc.squared_abs() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;

        if !ray_t.surrounds(root) {
            root = (h + sqrt_d) / a;

            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut rec = HitRecord::default();
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}

impl Into<HittableObject> for Sphere {
    fn into(self) -> HittableObject {
        Rc::new(self)
    }
}
