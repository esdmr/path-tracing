use crate::{
    f64::lerp,
    hittable::Hittable,
    interval::Interval,
    ppm::PPMImage,
    ray::Ray,
    vec3::{Color, Pos3, Vec3},
};

#[derive(Debug, Clone, Copy)]
pub struct CameraOptions {
    pub aspect_ratio: f64,
    pub image_width: usize,
}

impl Default for CameraOptions {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.,
            image_width: 100,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Camera {
    image_width: usize,
    image_height: usize,
    center: Pos3,
    pixel00_loc: Pos3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(options: CameraOptions) -> Self {
        let image_width = options.image_width;
        let image_height = (((image_width as f64) / options.aspect_ratio).trunc() as usize).max(1);

        // Camera

        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);
        let center = Pos3::default();

        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        let viewport_upper_left =
            center - Vec3::new(0., 0., focal_length) - (viewport_u + viewport_v) / 2.;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.;

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        if let Some(rec) = world.hit(r, Interval::new(0., f64::INFINITY)) {
            return (rec.normal + Color::new(1., 1., 1.)) / 2.;
        }

        let unit_direction = r.direction().normalize();
        let a = (unit_direction.y() + 1.) / 2.;
        lerp(a, Color::new(1., 1., 1.), Color::new(0.5, 0.7, 1.))
    }

    pub fn render(&self, world: &dyn Hittable) -> PPMImage {
        let mut image = PPMImage::new_empty(self.image_width, self.image_height);

        for y in 0..image.height() {
            eprint!(
                "\rScanlines remaining: {:4}/{:4}",
                image.height() - y,
                image.height()
            );

            for x in 0..image.width() {
                let ray = {
                    let center = self.pixel00_loc
                        + self.pixel_delta_u * (x as f64)
                        + self.pixel_delta_v * (y as f64);
                    Ray::new(center, center - self.center, x, y)
                };

                let color = Self::ray_color(&ray, world);
                image[(x, y)] = color.into();
            }
        }

        eprintln!("\nDone");
        image
    }
}
