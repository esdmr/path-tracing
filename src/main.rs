mod ppm;
mod ray;
mod vec3;

use std::ops::{Add, Mul};

use ppm::PPMImage;
use ray::Ray;
use vec3::{Color, Pos3, Vec3};

pub fn lerp<T1, T2, T3>(t: f64, a: T1, b: T1) -> T3
where
    T1: Mul<f64, Output = T2>,
    T2: Add<Output = T3>,
{
    a * (1. - t) + b * t
}

fn hit_sphere(center: &Pos3, radius: f64, r: &Ray) -> bool {
    let oc = center - r.origin();
    let a = r.direction().squared_abs();
    let b = -2. * r.direction().dot(&oc);
    let c = oc.squared_abs() - radius * radius;
    let discriminant = b * b - 4. * a * c;
    return discriminant >= 0.;
}

pub fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Pos3::new(0., 0., -1.), 0.5, r) {
        return Color::new(1., 0., 0.);
    }

    let unit_direction = r.direction().normalize();
    let a = (unit_direction.y() + 1.) / 2.;
    return lerp(a, Color::new(1., 1., 1.), Color::new(0.5, 0.7, 1.));
}

pub fn main() {
    let mut image = {
        let aspect_ratio = 16. / 9.;
        let width = 400;
        let height = ((width as f64) / aspect_ratio).trunc() as usize;
        PPMImage::new_empty(width, height.max(1))
    };

    // Camera

    let focal_length = 1.;
    let viewport_height = 2.;
    let viewport_width = viewport_height * (image.width() as f64) / (image.height() as f64);
    let camera_center = Pos3::default();

    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    let pixel_delta_u = viewport_u / (image.width() as f64);
    let pixel_delta_v = viewport_v / (image.height() as f64);

    let viewport_upper_left =
        camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;

    let pixel00_loc = viewport_upper_left + pixel_delta_u / 2. + pixel_delta_v / 2.;

    // Render

    for y in 0..image.height() {
        eprint!(
            "\rScanlines remaining: {:4}/{:4}",
            image.height() - y,
            image.height()
        );

        for x in 0..image.width() {
            let ray = {
                let center = pixel00_loc + pixel_delta_u * (x as f64) + pixel_delta_v * (y as f64);
                Ray::new(center, center - camera_center)
            };

            let color = ray_color(&ray);
            image[(x, y)] = color.into();
        }
    }

    eprintln!("\nDone");
    println!("{image}");
}
