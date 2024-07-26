use std::{fmt::Debug, sync::Arc};

use crate::{
    interval::Interval,
    material::MaterialObject,
    ray::Ray,
    vec3::{Pos3, Vec3},
};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Pos3,
    pub mat: MaterialObject,
    pub front_face: bool,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f64, p: Pos3, mat: MaterialObject, r: &Ray, outward_normal: Vec3) -> Self {
        let mut rec = HitRecord {
            t,
            p,
            mat,
            front_face: bool::default(),
            normal: Vec3::default(),
        };

        rec.set_face_normal(r, outward_normal);

        rec
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction().dot(&outward_normal) < 0.;

        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable: Debug {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub type HittableObject = Arc<dyn Hittable>;
