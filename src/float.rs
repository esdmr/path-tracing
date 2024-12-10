use std::ops::{Add, Mul};

pub type Fl = f64;

pub fn random() -> Fl {
    rand::random()
}

pub fn lerp<T1, T2, T3>(t: Fl, a: T1, b: T1) -> T3
where
    T1: Mul<Fl, Output = T2>,
    T2: Add<Output = T3>,
{
    a * (1. - t) + b * t
}
