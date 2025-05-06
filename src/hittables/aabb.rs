use crate::prelude::*;

#[derive(Clone, Debug, Copy)]
pub struct AABB {
    x : Interval,
    y : Interval,
    z : Interval,
}

impl AABB {
    #[allow(dead_code)]
    pub fn new(x : Interval, y : Interval, z : Interval) -> Self {
        Self {x, y, z}
    }

    pub fn universe() -> Self {
        Self {
            x : Interval::universe(),
            y : Interval::universe(),
            z : Interval::universe(),
        }
    }

    pub fn empty() -> Self {
        Self {
            x : Interval::empty(),
            y : Interval::empty(),
            z : Interval::empty(),
        }
    }

    pub fn from_points(a : Point3, b : Point3) -> Self {
        let x = if a.x <= b.x {Interval::new(a.x, b.x)} else {Interval::new(b.x, a.x)};
        let y = if a.y <= b.y {Interval::new(a.y, b.y)} else {Interval::new(b.y, a.y)};
        let z = if a.z <= b.z {Interval::new(a.z, b.z)} else {Interval::new(b.z, a.z)};
        Self {x, y, z}
    }

    pub fn from_boxes(a : AABB, b : AABB) -> Self {
        let x = Interval::combine(a.x, b.x);
        let y = Interval::combine(a.y, b.y);
        let z = Interval::combine(a.z, b.z);
        Self {x, y, z}
    }

    pub fn longest_axis(&self) -> i32 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {0} else {2}
        } else {
            if self.y.size() > self.z.size() {1} else {2}
        }
    }

    pub fn axis_interval(&self, n : i32) -> Interval {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Not a valid dimension"), 
        }
    }

    pub fn hit(&self, ray : &Ray) -> Option<Interval> {
        let origin = ray.origin;
        let dir = ray.direction;

        let mut hit_int = Interval::universe();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1. / dir[axis as usize];

            let t0 = (ax.min - origin[axis as usize]) * adinv;
            let t1 = (ax.max - origin[axis as usize]) * adinv;

            let (t0, t1) = if t0 < t1 {(t0, t1)} else {(t1, t0)};

            if t0 > hit_int.min {
                hit_int.min = t0;
            };

            if t1 < hit_int.max {
                hit_int.max = t1;
            };

            if hit_int.max <= hit_int.min {return None};
        }

        Some(hit_int)
    }
}