use std::fmt::Debug;

use crate::{
    hittable_list::HittableList, interval::Interval, material::MaterialObject, ray::Ray, sphere::Sphere, vec3::{Pos3, Vec3}
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

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

#[derive(Debug, Clone)]
pub enum HittableObject {
    Sphere(Sphere),
    List(HittableList),
}

impl From<Sphere> for HittableObject {
    fn from(val: Sphere) -> Self {
        HittableObject::Sphere(val)
    }
}

impl From<HittableList> for HittableObject {
    fn from(val: HittableList) -> Self {
        HittableObject::List(val)
    }
}

impl Hittable for HittableObject {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        match self {
            HittableObject::Sphere(i) => i.hit(r, ray_t),
            HittableObject::List(i) => i.hit(r, ray_t),
        }
    }
}
