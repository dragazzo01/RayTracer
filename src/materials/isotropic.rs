use crate::{prelude::*};

#[derive(Debug, Clone)]
pub struct Isotropic {
    tex: Arc<Textures>
}

impl Isotropic {
    pub fn solid(albedo: Color3) -> Self {
        Self {tex: Textures::solid_color(albedo)}
    }

    // pub fn new(tex: Arc<Textures>) -> Self {
    //     Self {tex}
    // }

    pub fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color3, Ray)> {
        let scattered = Ray::new_time(hit_record.point, Vec3::random_unit(rng), ray.time);
        let attenuation = self.tex.value(hit_record.u, hit_record.v, &hit_record.point);
        Some((attenuation, scattered))
    }
}

