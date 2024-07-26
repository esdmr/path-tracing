use std::{fmt::Debug, rc::Rc};

use crate::{
    f64::random,
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material: Debug {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let _ = r_in;
        let _ = rec;

        None
    }
}

pub type MaterialObject = Rc<dyn Material>;

// FIXME: Get rid of the default material

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct DefaultMaterial;

impl Material for DefaultMaterial {}

impl From<DefaultMaterial> for MaterialObject {
    fn from(value: DefaultMaterial) -> Self {
        Rc::new(value)
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct LambertianMaterial {
    albedo: Color,
}

impl LambertianMaterial {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = rec.normal + Vec3::random_normalized();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p, scatter_direction, r_in.pixel_x(), r_in.pixel_y()),
        })
    }
}

impl From<LambertianMaterial> for MaterialObject {
    fn from(value: LambertianMaterial) -> Self {
        Rc::new(value)
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct MetalMaterial {
    albedo: Color,
    fuzz: f64,
}

impl MetalMaterial {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected =
            r_in.direction().reflect(&rec.normal) + Vec3::random_normalized() * self.fuzz;
        let scattered = Ray::new(rec.p, reflected, r_in.pixel_x(), r_in.pixel_y());

        (scattered.direction().dot(&rec.normal) > 0.).then_some(ScatterRecord {
            attenuation: self.albedo,
            scattered,
        })
    }
}

impl From<MetalMaterial> for MaterialObject {
    fn from(value: MetalMaterial) -> Self {
        Rc::new(value)
    }
}

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
        Rc::new(value)
    }
}
