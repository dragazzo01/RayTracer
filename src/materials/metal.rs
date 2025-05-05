use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Metal {
    albedo : Color3,
    fuzz : f64,
}

impl Metal {
    pub(crate) fn new(albedo : Color3, fuzz : f64) -> Self {
        Self {albedo, fuzz}
    }

    pub(crate) fn scatter (&self, ray_in : &Ray, hit_record : &HitRecord, rng : &mut ThreadRng) -> Option<(Color3, Ray)> {
        let reflected = 
            ray_in.direction.reflect(&hit_record.normal).normalize() 
            + self.fuzz * Vec3::random_unit(rng);
        let scattered = Ray::new(hit_record.point, reflected);


        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
        
    }
}