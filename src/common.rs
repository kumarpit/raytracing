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

/// Real-Valued Interval utilities
#[derive(Default)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, v: f64) -> bool {
        self.min <= v && v <= self.max
    }

    pub fn surrounds(&self, v: f64) -> bool {
        self.min < v && v < self.max
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }
}

pub const EMPTY_INTERVAL: Interval = Interval {
    min: INFINITY,
    max: NEG_INFINITY,
};

pub const UNIVERSAL_INTERVAL: Interval = Interval {
    min: NEG_INFINITY,
    max: INFINITY,
};

pub const NON_NEGATIVE_INTERVAL: Interval = Interval {
    min: 0.0,
    max: INFINITY,
};

pub const NON_POSITIVE_INTERVAL: Interval = Interval {
    min: NEG_INFINITY,
    max: 0.0,
};
