mod ppm;
mod vec3;

use ppm::PPMImage;
use vec3::Color;

pub fn main() {
    let mut image = PPMImage::new_empty(256, 256);

    for y in 0..image.height() {
        eprint!(
            "\rScanlines remaining: {:4}/{:4}",
            image.height() - y,
            image.height()
        );

        for x in 0..image.width() {
            let color = Color::new(
                (x as f64) / ((image.width() - 1) as f64),
                (y as f64) / ((image.height() - 1) as f64),
                0.,
            );

            image[(x, y)] = color.into();
        }
    }

    eprintln!("\nDone");
    println!("{image}");
}
