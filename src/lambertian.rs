use crate::{
    hittable::HitRecord,
    material::{Material, ScatterRecord},
    ray::Ray,
    vec3::{Color, Vec3},
};

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
            scattered: Ray::new(rec.p, scatter_direction)
                .at_time_of(r_in)
                .for_pixel_of(r_in),
        })
    }
}
