use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub min : f64,
    pub max : f64,
}

#[allow(dead_code)]
impl Interval {
    pub fn new(min : f64, max : f64) -> Self {
        Self {min, max}
    }


    pub fn universe() -> Self {
        Self {
            min : NEG_INF,
            max : INF,
        }
    }

    pub fn empty() -> Self {
        Self {
            min : INF,
            max : NEG_INF,
        }
    }

    pub fn size(&self) -> f64 {
        self.max-self.min
    }
 
    pub fn contains(&self, x : f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x : f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x : f64) -> f64 {
        if x < self.min {self.min} 
        else if x > self.max {self.max} 
        else {x}
    }
}