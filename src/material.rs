use std::fmt::Debug;

use crate::{
    dielectric::DielectricMaterial, hittable::HitRecord, lambertian::LambertianMaterial,
    metal::MetalMaterial, ray::Ray, vec3::Color,
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material: Debug + Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let _ = r_in;
        let _ = rec;

        None
    }
}

#[derive(Debug, Clone)]
pub enum MaterialObject {
    Dielectric(DielectricMaterial),
    Lambertian(LambertianMaterial),
    Metal(MetalMaterial),
}

impl From<DielectricMaterial> for MaterialObject {
    fn from(value: DielectricMaterial) -> Self {
        MaterialObject::Dielectric(value)
    }
}

impl From<LambertianMaterial> for MaterialObject {
    fn from(value: LambertianMaterial) -> Self {
        MaterialObject::Lambertian(value)
    }
}

impl From<MetalMaterial> for MaterialObject {
    fn from(value: MetalMaterial) -> Self {
        MaterialObject::Metal(value)
    }
}

impl Material for MaterialObject {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        match self {
            MaterialObject::Dielectric(i) => i.scatter(r_in, rec),
            MaterialObject::Lambertian(i) => i.scatter(r_in, rec),
            MaterialObject::Metal(i) => i.scatter(r_in, rec),
        }
    }
}
