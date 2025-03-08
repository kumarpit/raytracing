/**
 * Constants
 */
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;
pub use std::f64::NEG_INFINITY;
use std::ops::{Add, Mul};

use rand::Rng;

/**
 * Math utilities
 */
#[inline]
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

/// Linear Interpolation
#[inline]
pub fn lerp<T>(start: T, end: T, t: f64) -> T
where
    T: Mul<f64, Output = T> + Add<T, Output = T>,
{
    start * (1.0 - t) + end * t
}

/// Returns a random real in range [0.0, 1.0)
#[inline]
pub fn random() -> f64 {
    rand::thread_rng().gen()
}

/// Returns a random real in range [min, max)
#[inline]
pub fn random_in_range(min: f64, max: f64) -> f64 {
    lerp(min, max, random())
}

#[inline]
pub fn clamp(min: f64, max: f64, v: f64) -> f64 {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}
