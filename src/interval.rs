use crate::common::INFINITY;
use crate::common::NEG_INFINITY;

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

pub const POSITIVE_INTERVAL: Interval = Interval {
    min: 0.0,
    max: INFINITY,
};

pub const NEGATIVE_INTERVAL: Interval = Interval {
    min: NEG_INFINITY,
    max: 0.0,
};

pub const ANTI_SHADOW_ACNE_HIT_INTERVAL: Interval = Interval {
    min: 0.001,
    max: INFINITY,
};
