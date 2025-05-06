use crate::prelude::*;
#[derive(Debug, Clone)]
pub enum Textures {
    Checkered(CheckerTexture),
    Solid(Solid),
}

impl Textures {
    pub fn solid_color(albedo : Color3) -> Arc<Self> {
        Arc::new(Self::Solid(Solid {albedo}))
    }

    pub fn rgb(red : f64, green : f64, blue : f64) -> Arc<Self> {
        Self::solid_color(Color3::new(red, green, blue))
    }

    pub fn checker(scale: f64, even: Arc<Textures>, odd: Arc<Textures>) -> Arc<Self> {
        Arc::new(Self::Checkered(CheckerTexture {inv_scale: 1. / scale, even, odd}))
    }

    pub fn value(&self, u : f64, v : f64, p : &Point3) -> Color3 {
        match self {
            Self::Checkered(c) => c.value(u, v, p),
            Self::Solid(s) => s.value(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Solid {
    albedo : Color3
}

impl Solid {
    pub fn value(&self) -> Color3 {
        self.albedo
    }
}
#[derive(Debug, Clone)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<Textures>,
    odd: Arc<Textures>,
}

impl CheckerTexture {
    pub fn value(&self, u : f64, v : f64, p : &Point3) -> Color3 { 
        let x  = (self.inv_scale * p.x).floor() as i32;
        let y  = (self.inv_scale * p.y).floor() as i32;
        let z  = (self.inv_scale * p.z).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}