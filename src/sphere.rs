use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable, HittableObject},
    interval::Interval,
    material::MaterialObject,
    ray::Ray,
    vec3::Pos3,
};

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Pos3,
    radius: f64,
    mat: MaterialObject,
}

impl Sphere {
    pub fn new(center: Pos3, radius: f64, mat: MaterialObject) -> Self {
        Self {
            center,
            radius,
            mat,
        }
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

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;

        Some(HitRecord::new(t, p, self.mat.clone(), r, outward_normal))
    }
}

impl From<Sphere> for HittableObject {
    fn from(val: Sphere) -> Self {
        Rc::new(val)
    }
}
