use crate::vec3::{Pos3, Vec3};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Ray {
    origin: Pos3,
    direction: Vec3,
    pixel_x: usize,
    pixel_y: usize,
}

impl Ray {
    pub const fn new(origin: Pos3, direction: Vec3, pixel_x: usize, pixel_y: usize) -> Self {
        Self {
            origin,
            direction,
            pixel_x,
            pixel_y,
        }
    }

    pub const fn origin(&self) -> &Pos3 {
        &self.origin
    }

    pub const fn direction(&self) -> &Vec3 {
        &self.direction
    }

	pub const fn is_pixel_at(&self, x: usize, y: usize) -> bool {
		self.pixel_x == x && self.pixel_y == y
	}

    pub fn at(&self, t: f64) -> Pos3 {
        self.origin + self.direction * t
    }
}
