use std::{fmt::Display, ops::{Index, IndexMut}};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

#[derive(Debug, Clone)]
pub struct PPMImage {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl PPMImage {
    pub fn new(width: usize, height: usize, mut pixels: Vec<Color>) -> Self {
        pixels.resize_with(width * height, Default::default);

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn new_empty(width: usize, height: usize) -> Self {
		Self {
            width,
            height,
            pixels: vec![Color::default(); width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Index<usize> for PPMImage {
	type Output = Color;

	fn index(&self, index: usize) -> &Self::Output {
		&self.pixels[index]
	}
}

impl IndexMut<usize> for PPMImage {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.pixels[index]
	}
}

impl Index<(usize, usize)> for PPMImage {
	type Output = Color;

	fn index(&self, index: (usize, usize)) -> &Self::Output {
		&self.pixels[index.0 + index.1 * self.width]
	}
}

impl IndexMut<(usize, usize)> for PPMImage {
	fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
		&mut self.pixels[index.0 + index.1 * self.width]
	}
}

impl Display for PPMImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert!(self.pixels.len() == self.width * self.height);

        write!(f, "P3\n{} {}\n255\n", self.width, self.height)?;

        for pixel in &self.pixels {
            write!(f, "{} {} {}\n", pixel.r, pixel.g, pixel.b)?;
        }

        Ok(())
    }
}
