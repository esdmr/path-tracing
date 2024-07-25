use std::ops::{Add, Mul};

pub fn lerp<T1, T2, T3>(t: f64, a: T1, b: T1) -> T3
where
    T1: Mul<f64, Output = T2>,
    T2: Add<Output = T3>,
{
    a * (1. - t) + b * t
}
