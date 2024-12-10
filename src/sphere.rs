use crate::{
    float::Fl, hittable::{HitRecord, Hittable}, interval::Interval, material::MaterialObject, ray::Ray, vec3::Pos3
};

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Ray,
    radius: Fl,
    mat: MaterialObject,
}

impl Sphere {
    pub fn new(center: Pos3, radius: Fl, mat: MaterialObject) -> Self {
        Self {
            center: Ray::new(center, Pos3::default()),
            radius: radius.max(0.),
            mat,
        }
    }

    pub fn new_moving(center1: Pos3, center2: Pos3, radius: Fl, mat: MaterialObject) -> Self {
        Self {
            center: Ray::new(center1, center2 - center1),
            radius: radius.max(0.),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(r.time());
        let oc = &current_center - r.origin();
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
        let outward_normal = (p - current_center) / self.radius;

        Some(HitRecord::new(t, p, self.mat.clone(), r, outward_normal))
    }
}
