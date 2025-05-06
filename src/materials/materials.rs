use crate::prelude::*;
use crate::materials::lambertian::Lambertian;
use crate::materials::dielectric::Dielectric;
use crate::materials::metal::Metal;

#[derive(Debug, Copy, Clone)]
pub enum Materials {
    Lambertian(Lambertian),
    Dielectric(Dielectric),
    Metal(Metal),
}

impl Materials {
    pub fn lambertian(albedo : Color3) -> Self {
        Self::Lambertian(Lambertian::new(albedo))
    }

    pub fn metal(albedo : Color3, fuzz : f64) -> Self {
        let fuzz = if fuzz < 1.0 {fuzz} else {1.0};
        Self::Metal(Metal::new(albedo, fuzz))
    }

    pub fn dielectric(refraction_index : f64) -> Self {
        Self::Dielectric(Dielectric::new(refraction_index))
    }

    pub fn scatter(&self, ray : &Ray, hit_record : &HitRecord, rng : &mut ThreadRng) -> Option<(Color3, Ray)> {
        match self {
            Materials::Lambertian(l) => l.scatter(ray, hit_record, rng),
            Materials::Dielectric(d) => d.scatter(ray, hit_record, rng),
            Materials::Metal(m) => m.scatter(ray, hit_record, rng),
        }
    }
}



