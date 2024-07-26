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
use f64::random;
use hittable_list::HittableList;
use interval::Interval;
use material::{DielectricMaterial, LambertianMaterial, MaterialObject, MetalMaterial};
use sphere::Sphere;
use vec3::{Color, Pos3, Vec3};

pub fn main() {
    let mut world = HittableList::default();

    let ground_material: MaterialObject = LambertianMaterial::new(Color::new(0.5, 0.5, 0.5)).into();
    world.add(Sphere::new(Pos3::new(0., -1000., 0.), 1000., ground_material).into());

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Pos3::new(
                (a as f64) + 0.9 * random(),
                0.2,
                (b as f64) + 0.9 * random(),
            );

            if choose_mat < 0.8 {
                world.add(
                    Sphere::new(center, 0.2, LambertianMaterial::new(Color::random()).into())
                        .into(),
                );
            } else if choose_mat < 0.95 {
                world.add(
                    Sphere::new(
                        center,
                        0.2,
                        MetalMaterial::new(
                            Color::random_between(Interval::new(0.5, 1.)),
                            Interval::new(0., 0.5).random(),
                        )
                        .into(),
                    )
                    .into(),
                );
            } else {
                world.add(Sphere::new(center, 0.2, DielectricMaterial::new(1.5).into()).into());
            }
        }
    }

    world.add(
        Sphere::new(
            Pos3::new(0., 1., 0.),
            1.,
            DielectricMaterial::new(1.5).into(),
        )
        .into(),
    );

    world.add(
        Sphere::new(
            Pos3::new(-4., 1., 0.),
            1.,
            LambertianMaterial::new(Color::new(0.4, 0.2, 0.1)).into(),
        )
        .into(),
    );

    world.add(
        Sphere::new(
            Pos3::new(4., 1., 0.),
            1.,
            MetalMaterial::new(Color::new(0.7, 0.6, 0.5), 0.).into(),
        )
        .into(),
    );

    let camera = Camera::new(CameraOptions {
        aspect_ratio: 16. / 9.,
        image_width: 1200,
        samples_per_pixel: 50,
        max_depth: 50,
        v_fov: 20.,
        look_from: Pos3::new(13., 2., 3.),
        look_at: Pos3::new(0., 0., -0.),
        vup: Vec3::new(0., 1., 0.),
        defocus_angle: 0.6,
        focus_dist: 10.,
    });

    let image = camera.render(&world);

    println!("{image}");
}
