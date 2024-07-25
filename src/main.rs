mod camera;
mod f64;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ppm;
mod ray;
mod sphere;
mod vec3;
use camera::{Camera, CameraOptions};
use hittable_list::HittableList;
use material::{LambertianMaterial, MaterialObject, MetalMaterial};
use sphere::Sphere;
use vec3::{Color, Pos3};

pub fn main() {
    let mut world = HittableList::default();

    let material_ground: MaterialObject = LambertianMaterial::new(Color::new(0.8, 0.8, 0.)).into();
    let material_center: MaterialObject = LambertianMaterial::new(Color::new(0.1, 0.2, 0.5)).into();
    let material_left: MaterialObject = MetalMaterial::new(Color::new(0.8, 0.8, 0.8), 0.3).into();
    let material_right: MaterialObject = MetalMaterial::new(Color::new(0.8, 0.6, 0.2), 1.).into();

    world.add(Sphere::new(Pos3::new(0., -100.5, -1.), 100., material_ground).into());
    world.add(Sphere::new(Pos3::new(0., 0., -1.2), 0.5, material_center).into());
    world.add(Sphere::new(Pos3::new(-1., 0., -1.), 0.5, material_left).into());
    world.add(Sphere::new(Pos3::new(1., 0., -1.), 0.5, material_right).into());

    let camera = Camera::new(CameraOptions {
        aspect_ratio: 16. / 9.,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
    });

    let image = camera.render(&world);

    println!("{image}");
}
