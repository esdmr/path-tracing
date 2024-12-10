use crate::float::{lerp, random, Fl};

pub struct Interval {
    min: Fl,
    max: Fl,
}

impl Interval {
    pub const fn new(min: Fl, max: Fl) -> Self {
        Self { min, max }
    }

    pub const fn new_empty() -> Self {
        Self {
            min: Fl::INFINITY,
            max: Fl::NEG_INFINITY,
        }
    }

    pub const fn new_universe() -> Self {
        Self {
            min: Fl::NEG_INFINITY,
            max: Fl::INFINITY,
        }
    }

    pub const fn get_min(&self) -> Fl {
        self.min
    }

    pub const fn get_max(&self) -> Fl {
        self.max
    }

    pub fn size(&self) -> Fl {
        self.max - self.min
    }

    pub fn contains(&self, x: Fl) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: Fl) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: Fl) -> Fl {
        x.clamp(self.min, self.max)
    }

    pub fn random(&self) -> Fl {
        lerp(random(), self.get_min(), self.get_max())
    }
}
