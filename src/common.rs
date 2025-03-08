/// Constants
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;
use std::ops::{Add, Mul};

/// Math utilities
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

/// Linear Interpolation
pub fn lerp<T>(start: T, end: T, t: f64) -> T
where
    T: Mul<f64, Output = T> + Add<T, Output = T>,
{
    start * (1.0 - t) + end * t
}

/// Checks if t_min <= t <= t_max
pub fn is_in_range(t_min: f64, t_max: f64, t: f64) -> bool {
    t >= t_min && t <= t_max
}
