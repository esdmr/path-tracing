use crate::{float::Fl, vec3::{Pos3, Vec3}};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Ray {
    origin: Pos3,
    direction: Vec3,
    time: Fl,
    pixel: Option<(usize, usize)>,
}

impl Ray {
    pub const fn new(origin: Pos3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            time: 0.,
            pixel: None,
        }
    }

    pub const fn origin(&self) -> &Pos3 {
        &self.origin
    }

    pub const fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub const fn at_time(&self, time: Fl) -> Self {
        Self {
            direction: self.direction,
            origin: self.origin,
            time,
            pixel: self.pixel,
        }
    }

    pub const fn at_time_of(&self, other: &Self) -> Self {
        self.at_time(other.time)
    }

    pub fn at_time_mut(&mut self, time: Fl) {
        self.time = time;
    }

    pub const fn time(&self) -> Fl {
        self.time
    }

    pub const fn for_pixel(&self, pixel: (usize, usize)) -> Self {
        Self {
            direction: self.direction,
            origin: self.origin,
            time: self.time,
            pixel: Some(pixel),
        }
    }

    pub const fn for_pixel_of(&self, other: &Self) -> Self {
        if let Some(p) = other.pixel {
            self.for_pixel(p)
        } else {
            *self
        }
    }

    pub fn for_pixel_mut(&mut self, pixel: (usize, usize)) {
        self.pixel = Some(pixel);
    }

    pub const fn pixel(&self) -> Option<(usize, usize)> {
        self.pixel
    }

    pub fn at(&self, t: Fl) -> Pos3 {
        self.origin + self.direction * t
    }
}
