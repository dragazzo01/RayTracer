use crate::prelude::*;

/// Represents a ray in 3D space, defined by an origin point, a direction vector, and an optional time parameter.
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    /// The origin point of the ray.
    pub origin: Point3,
    /// The direction vector of the ray.
    pub direction: Vec3,
    /// The time parameter associated with the ray (useful for motion blur or time-dependent calculations).
    pub time: f64,
}

impl Ray {
    /// Creates a new `Ray` with the given origin and direction.
    ///
    /// # Arguments
    ///
    /// * `origin` - The starting point of the ray.
    /// * `direction` - The direction vector of the ray.
    ///
    /// # Returns
    ///
    /// A new `Ray` instance with the specified origin and direction, and a default time of `0.0`.
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            time: 0.0,
        }
    }

    /// Creates a new `Ray` with the given origin, direction, and time.
    ///
    /// # Arguments
    ///
    /// * `origin` - The starting point of the ray.
    /// * `direction` - The direction vector of the ray.
    /// * `time` - The time parameter associated with the ray.
    ///
    /// # Returns
    ///
    /// A new `Ray` instance with the specified origin, direction, and time.
    pub fn new_time(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    /// Computes the point along the ray at a given parameter `t`.
    ///
    /// # Arguments
    ///
    /// * `t` - The parameter along the ray's direction vector.
    ///
    /// # Returns
    ///
    /// The point at the specified parameter `t` along the ray.
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
