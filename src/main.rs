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
use material::{DielectricMaterial, LambertianMaterial, MaterialObject, MetalMaterial};
use sphere::Sphere;
use vec3::{Color, Pos3, Vec3};

pub fn main() {
    let mut world = HittableList::default();

    let material_ground: MaterialObject = LambertianMaterial::new(Color::new(0.8, 0.8, 0.)).into();
    let material_center: MaterialObject = LambertianMaterial::new(Color::new(0.1, 0.2, 0.5)).into();
    let material_left: MaterialObject = DielectricMaterial::new(1.50).into();
    let material_bubble: MaterialObject = DielectricMaterial::new(1. / 1.50).into();
    let material_right: MaterialObject = MetalMaterial::new(Color::new(0.8, 0.6, 0.2), 1.).into();

    world.add(Sphere::new(Pos3::new(0., -100.5, -1.), 100., material_ground).into());
    world.add(Sphere::new(Pos3::new(0., 0., -1.2), 0.5, material_center).into());
    world.add(Sphere::new(Pos3::new(-1., 0., -1.), 0.5, material_left).into());
    world.add(Sphere::new(Pos3::new(-1., 0., -1.), 0.4, material_bubble).into());
    world.add(Sphere::new(Pos3::new(1., 0., -1.), 0.5, material_right).into());

    let camera = Camera::new(CameraOptions {
        aspect_ratio: 16. / 9.,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        v_fov: 20.,
        look_from: Pos3::new(-2., 2., 1.),
        look_at: Pos3::new(0., 0., -1.),
        vup: Vec3::new(0., 1., 0.),
        defocus_angle: 10.,
        focus_dist: 3.4,
    });

    let image = camera.render(&world);

    println!("{image}");
}
