use crate::{
    f64::{lerp, random},
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
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub v_fov: f64,
    pub look_from: Pos3,
    pub look_at: Pos3,
    pub vup: Vec3,
}

impl Default for CameraOptions {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            v_fov: 90.,
            look_from: Pos3::default(),
            look_at: Pos3::new(0., 0., -1.),
            vup: Pos3::new(0., 1., 0.),
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
    u: Vec3,
    v: Vec3,
    w: Vec3,
    samples_per_pixel: usize,
    pixel_samples_scale: f64,
    max_depth: usize,
    v_fov: f64,
}

impl Camera {
    pub fn new(options: CameraOptions) -> Self {
        let image_width = options.image_width;
        let image_height = (((image_width as f64) / options.aspect_ratio).trunc() as usize).max(1);

        // Camera

        let v_fov = options.v_fov;
        let focal_length = 1.;
        let theta = v_fov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * focal_length;
        let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);
        let center = options.look_from;

        let w = (options.look_from - options.look_at).normalize();
        let u = options.vup.cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_u = u * viewport_width;
        let viewport_v = v * -viewport_height;

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        let viewport_upper_left =
            center - w * focal_length - (viewport_u + viewport_v) / 2.;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.;

        // Anti-aliasing

        let samples_per_pixel = options.samples_per_pixel;
        let pixel_samples_scale = 1. / (samples_per_pixel as f64);

        // Diffuse

        let max_depth = options.max_depth;

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
            v_fov,
        }
    }

    fn get_ray(&self, x: usize, y: usize) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * ((x as f64) + offset.x()))
            + (self.pixel_delta_v * ((y as f64) + offset.y()));

        Ray::new(self.center, pixel_sample - self.center, x, y)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random() - 0.5, random() - 0.5, 0.)
    }

    fn ray_color(r: &Ray, depth: usize, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            if let Some(rec) = rec.mat.scatter(r, &rec) {
                return Self::ray_color(&rec.scattered, depth - 1, world) * rec.attenuation;
            }

            return Color::default();
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
                let mut color = Color::default();

                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color += Self::ray_color(&ray, self.max_depth, world);
                }

                image[(x, y)] = (color * self.pixel_samples_scale).into();
            }
        }

        eprintln!("\nDone");
        image
    }
}
