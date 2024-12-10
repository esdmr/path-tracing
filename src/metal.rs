use crate::{
    float::Fl,
    hittable::HitRecord,
    material::{Material, ScatterRecord},
    ray::Ray,
    vec3::{Color, Vec3},
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct MetalMaterial {
    albedo: Color,
    fuzz: Fl,
}

impl MetalMaterial {
    pub fn new(albedo: Color, fuzz: Fl) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected =
            r_in.direction().reflect(&rec.normal) + Vec3::random_normalized() * self.fuzz;
        let scattered = Ray::new(rec.p, reflected)
            .at_time_of(r_in)
            .for_pixel_of(r_in);

        (scattered.direction().dot(&rec.normal) > 0.).then_some(ScatterRecord {
            attenuation: self.albedo,
            scattered,
        })
    }
}
