use crate::prelude::*;

/// Represents a closed interval [min, max] in the real number line.
#[derive(Debug, Copy, Clone)]
pub struct Interval {
    /// The minimum value of the interval.
    pub min: f64,
    /// The maximum value of the interval.
    pub max: f64,
}

#[allow(dead_code)]
impl Interval {
    /// Creates a new interval with the given minimum and maximum values.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value of the interval.
    /// * `max` - The maximum value of the interval.
    ///
    /// # Returns
    ///
    /// A new `Interval` instance.
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Combines two intervals into a new interval that spans both.
    ///
    /// # Arguments
    ///
    /// * `a` - The first interval.
    /// * `b` - The second interval.
    ///
    /// # Returns
    ///
    /// A new `Interval` that spans the union of `a` and `b`.
    pub fn combine(a: Self, b: Self) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };
        Self { min, max }
    }

    /// Returns an interval that represents the entire real number line.
    ///
    /// # Returns
    ///
    /// An `Interval` for \[-∞, ∞\].
    pub fn universe() -> Self {
        Self {
            min: NEG_INF,
            max: INF,
        }
    }

    /// Returns an empty interval.
    ///
    /// # Returns
    ///
    /// An empty `Interval` .
    pub fn empty() -> Self {
        Self {
            min: INF,
            max: NEG_INF,
        }
    }

    /// Expands the interval by a given delta.
    ///
    /// # Arguments
    ///
    /// * `delta` - The amount by which to expand the interval.
    ///
    /// # Returns
    ///
    /// For original `Interval : [x, y]` returns `[x - delta/2, y + delta/2]`.
    pub fn expand(&mut self, delta: f64) {
        let padding = delta / 2.;
        self.min -= padding;
        self.max += padding;
    }

    /// Calculates the size (length) of the interval.
    ///
    /// # Returns
    ///
    /// The size of the interval as a `f64`.
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Checks if a value is contained within the interval.
    ///
    /// # Arguments
    ///
    /// * `x` - The value to check.
    ///
    /// # Returns
    ///
    /// `true` if `x` is within the interval, `false` otherwise.
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Checks if a value is strictly surrounded by the interval.
    ///
    /// # Arguments
    ///
    /// * `x` - The value to check.
    ///
    /// # Returns
    ///
    /// `true` if `x` is strictly within the interval, `false` otherwise.
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// Clamps a value to the nearest boundary of the interval.
    ///
    /// # Arguments
    ///
    /// * `x` - The value to clamp.
    ///
    /// # Returns
    ///
    /// The clamped value.
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
