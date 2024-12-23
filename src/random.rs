use crate::*;

pub fn random_01() -> f64 {
    rand::random::<f64>()
}

pub fn random_bound(min : f64, max : f64) -> f64 {
    min + (max - min)*random_01()
}

pub fn gen_01(rng : &mut ThreadRng) -> f64 {
    rng.gen()
}

pub fn gen_bound(min : f64, max : f64, rng : &mut ThreadRng) -> f64 {
    min + (max - min)*gen_01(rng)
}


