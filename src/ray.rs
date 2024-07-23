use crate::vec3::{Pos3, Vec3};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Ray {
	origin: Pos3,
	direction: Vec3,
}

impl Ray {
	pub fn new(origin: Pos3, direction: Vec3) -> Self {
		Self {origin, direction}
	}

	pub fn origin(&self) -> &Pos3 {
		&self.origin
	}

	pub fn direction(&self) -> &Vec3 {
		&self.direction
	}

	pub fn at(&self, t: f64) -> Pos3 {
		self.origin + self.direction * t
	}
}
