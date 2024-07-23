mod ppm;

use ppm::{PPMColor, PPMImage};

pub fn main() {
    let mut image = PPMImage::new_empty(256, 256);

    for y in 0..image.height() {
        eprint!(
            "\rScanlines remaining: {:4}/{:4}",
            image.height() - y,
            image.height()
        );

        for x in 0..image.width() {
            let r = (x as f64) / (image.width() as f64) * 256.;
            let g = (y as f64) / (image.height() as f64) * 256.;

            image[(x, y)] = PPMColor::new(r.trunc() as u8, g.trunc() as u8, 0);
        }
    }

    eprintln!("\nDone");
    println!("{image}");
}
