use std::sync::Arc;

use crate::{
    f64::random,
    hittable::HitRecord,
    material::{Material, MaterialObject, ScatterRecord},
    ray::Ray,
    vec3::Color,
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct DielectricMaterial {
    refraction_index: f64,
}

impl DielectricMaterial {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1. - refraction_index) / (1. + refraction_index);
        r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let ri = if rec.front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction().normalize();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let direction = if ri * sin_theta > 1. || Self::reflectance(cos_theta, ri) > random() {
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(&rec.normal, ri)
        };

        Some(ScatterRecord {
            attenuation: Color::new(1., 1., 1.),
            scattered: Ray::new(rec.p, direction, r_in.pixel_x(), r_in.pixel_y()),
        })
    }
}

impl From<DielectricMaterial> for MaterialObject {
    fn from(value: DielectricMaterial) -> Self {
        Arc::new(value)
    }
}
