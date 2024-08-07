use std::{fmt::Debug, sync::Arc};

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::Color,
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

pub type MaterialObject = Arc<dyn Material>;
