use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Lambertian {
    albedo : Color3,
}

impl Lambertian {
    pub(crate) fn new(albedo : Color3) -> Self {
        Self {albedo}
    }

    pub(crate) fn scatter (&self, hit_record : &HitRecord, rng : &mut ThreadRng) -> Option<(Color3, Ray)> {
        let scatter_direction = {
            let res = hit_record.normal + Vec3::random_unit(rng);
            if res.near_zero() {
                hit_record.normal
            } else {
                res
            }
        };
        

        let scattered = Ray::new(hit_record.point, scatter_direction);
        Some((self.albedo, scattered))
    }
}