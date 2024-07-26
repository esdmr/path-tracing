use indicatif::ProgressIterator;

use crate::{
    f64::{lerp, random},
    hittable::Hittable,
    interval::Interval,
    ppm::{PPMColor, PPMImage},
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
    pub defocus_angle: f64,
    pub focus_dist: f64,
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
            defocus_angle: 0.,
            focus_dist: 10.,
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
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    samples_per_pixel: usize,
    pixel_samples_scale: f64,
    max_depth: usize,
}

impl Camera {
    pub fn new(options: CameraOptions) -> Self {
        let image_width = options.image_width;
        let image_height = (((image_width as f64) / options.aspect_ratio).trunc() as usize).max(1);

        // Camera

        let theta = options.v_fov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * options.focus_dist;
        let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);
        let center = options.look_from;

        let w = (options.look_from - options.look_at).normalize();
        let u = options.vup.cross(&w).normalize();
        let v = w.cross(&u);

        let viewport_u = u * viewport_width;
        let viewport_v = v * -viewport_height;

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        let viewport_upper_left = center - w * options.focus_dist - (viewport_u + viewport_v) / 2.;

        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.;

        // Anti-aliasing

        let samples_per_pixel = options.samples_per_pixel;
        let pixel_samples_scale = 1. / (samples_per_pixel as f64);

        // Diffuse

        let max_depth = options.max_depth;

        // Defocus Blur

        let defocus_angle = options.defocus_angle;
        let defocus_radius = options.focus_dist * (defocus_angle / 2.).to_radians().tan();

        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
        }
    }

    fn get_ray(&self, x: usize, y: usize) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * ((x as f64) + offset.x()))
            + (self.pixel_delta_v * ((y as f64) + offset.y()));

        let origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.sample_defocus_disk()
        };

        Ray::new(origin, pixel_sample - origin, x, y)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random() - 0.5, random() - 0.5, 0.)
    }

    fn sample_defocus_disk(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + self.defocus_disk_u * p.x() + self.defocus_disk_v * p.y()
    }

    fn ray_color(r: &Ray, depth: usize, world: &dyn Hittable) -> Color {
        if depth == 0 {
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

    fn render_pixel(&self, world: &dyn Hittable, x: usize, y: usize) -> PPMColor {
        let mut color = Color::default();

        for _sample in 0..self.samples_per_pixel {
            let ray = self.get_ray(x, y);
            color += Self::ray_color(&ray, self.max_depth, world);
        }

        (color * self.pixel_samples_scale).into()
    }

    pub fn render(&self, world: &dyn Hittable) -> PPMImage {
        let mut image = PPMImage::new(self.image_width, self.image_height);

        for y in (0..image.height()).progress() {
            for x in 0..image.width() {
                image[(x, y)] = self.render_pixel(world, x, y);
            }
        }

        image
    }
}
