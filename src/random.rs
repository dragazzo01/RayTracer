use crate::prelude::*;

pub fn gen_01(rng : &mut ThreadRng) -> f64 {
    rng.gen()
}

pub fn gen_bound(min : f64, max : f64, rng : &mut ThreadRng) -> f64 {
    min + (max - min)*gen_01(rng)
}

pub fn gen_int(min : i32, max : i32, rng : &mut ThreadRng) -> i32 {
    rng.gen_range(min..max)
}


