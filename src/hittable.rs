use std::{fmt::Debug, rc::Rc};

use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Pos3, Vec3},
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct HitRecord {
    pub p: Pos3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
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

pub type HittableObject = Rc<dyn Hittable>;
