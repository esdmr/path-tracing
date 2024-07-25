mod camera;
mod f64;
mod hittable;
mod hittable_list;
mod interval;
mod ppm;
mod ray;
mod sphere;
mod vec3;
use camera::{Camera, CameraOptions};
use hittable_list::HittableList;
use sphere::Sphere;
use vec3::Pos3;

pub fn main() {
    let mut world = HittableList::default();

    world.add(Sphere::new(Pos3::new(0., 0., -2.), 0.5).into());
    world.add(Sphere::new(Pos3::new(0., -101., -2.), 100.).into());

    let camera = Camera::new(CameraOptions {
        aspect_ratio: 16. / 9.,
        image_width: 400,
        samples_per_pixel: 100,
    });

    let image = camera.render(&world);

    println!("{image}");
}
