use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Diffuse {
    texture: Arc<Textures>,
}

impl Diffuse {
    pub fn new(texture : Arc<Textures>) -> Self {
        Self {texture}
    }

    pub fn emitted(&self, u : f64, v : f64, p : &Point3) -> Color3 {
        self.texture.value(u, v, p)
    }
}