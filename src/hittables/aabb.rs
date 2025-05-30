use crate::prelude::*;

/// Represents an Axis-Aligned Bounding Box (AABB).
///
/// # Fields
/// - `x`: The interval along the x-axis.
/// - `y`: The interval along the y-axis.
/// - `z`: The interval along the z-axis.
#[derive(Clone, Debug, Copy)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    /// Creates a new AABB with specified intervals for each axis.
    ///
    /// # Arguments
    /// - `x`: Interval along the x-axis.
    /// - `y`: Interval along the y-axis.
    /// - `z`: Interval along the z-axis.
    ///
    /// # Returns
    /// A new `AABB` instance.
    #[allow(dead_code)]
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn offset(&self, offset: Vec3) -> Self {
        Self {
            x: self.x.offset(offset.x),
            y: self.y.offset(offset.y),
            z: self.z.offset(offset.z),
        }
    }

    /// Returns an empty AABB.
    ///
    /// # Returns
    /// An `AABB` with empty intervals on all axes.
    pub fn empty() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    /// Creates an AABB that encloses two points.
    ///
    /// # Arguments
    /// - `a`: The first point.
    /// - `b`: The second point.
    ///
    /// # Returns
    /// An `AABB` that encloses the two points.
    pub fn from_points(a: Point3, b: Point3) -> Self {
        let x = if a.x <= b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        };
        let y = if a.y <= b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        };
        let z = if a.z <= b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        };
        Self { x, y, z }
    }

    /// Combines two AABBs into one that encloses both.
    ///
    /// # Arguments
    /// - `a`: The first AABB.
    /// - `b`: The second AABB.
    ///
    /// # Returns
    /// An `AABB` that encloses both input AABBs.
    pub fn from_boxes(a: &AABB, b: &AABB) -> Self {
        let x = Interval::combine(a.x, b.x);
        let y = Interval::combine(a.y, b.y);
        let z = Interval::combine(a.z, b.z);
        Self { x, y, z }
    }

    /// Determines the longest axis of the AABB.
    ///
    /// # Returns
    /// An integer representing the longest axis:
    /// - `0` for x-axis
    /// - `1` for y-axis
    /// - `2` for z-axis
    pub fn longest_axis(&self) -> i32 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }

    /// Returns the interval along a specified axis.
    ///
    /// # Arguments
    /// - `n`: The axis index (0 for x, 1 for y, 2 for z).
    ///
    /// # Returns
    /// The interval along the specified axis.
    ///
    /// # Panics
    /// Panics if the axis index is not valid.
    pub fn axis_interval(&self, n: i32) -> Interval {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Not a valid dimension"),
        }
    }

    /// Determines if a ray intersects the AABB.
    ///
    /// # Arguments
    /// - `ray`: The ray to test for intersection.
    ///
    /// # Returns
    /// An `Option<Interval>` containing the intersection interval if the ray hits the AABB,
    /// or `None` if there is no intersection.
    pub fn hit(&self, ray: &Ray) -> Option<Interval> {
        let origin = ray.origin;
        let dir = ray.direction;

        let mut hit_int = Interval::universe();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1. / dir[axis as usize];

            let t0 = (ax.min - origin[axis as usize]) * adinv;
            let t1 = (ax.max - origin[axis as usize]) * adinv;

            let (t0, t1) = if t0 < t1 { (t0, t1) } else { (t1, t0) };

            if t0 > hit_int.min {
                hit_int.min = t0;
            };

            if t1 < hit_int.max {
            
    hit_int.max = t1;
            };

            if hit_int.max <= hit_int.min {
                return None;
            };
        }

        Some(hit_int)
    }


    pub fn pad_to_minimums(&mut self) {
        // Adjust the AABB so that no side is narrower than some delta, padding if necessary.

        let delta = 0.0001;
        if self.x.size() < delta {self.x.expand(delta);}
        if self.y.size() < delta {self.y.expand(delta);}
        if self.z.size() < delta {self.z.expand(delta);}
    }

}
