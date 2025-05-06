use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub(crate) struct Dielectric {
    refraction_index : f64,
}

fn reflectance(cosine : f64, refraction_index : f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0)*f64::powf(1.0 - cosine, 5.0)
}

impl Dielectric {
    pub(crate) fn new(refraction_index : f64) -> Self {
        Self {refraction_index}
    }

    pub(crate) fn scatter(&self, ray_in : &Ray, hit_record : &HitRecord, rng : &mut ThreadRng) -> Option<(Color3, Ray)> {
        let ri = if hit_record.front_face {1.0 / self.refraction_index} else {self.refraction_index};
        let unit_direction = ray_in.direction.normalize();
    
        let cos_theta = f64::min(-1.0 * unit_direction.dot(&hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let direction = 
        if ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > gen_01(rng) {
            unit_direction.reflect(&hit_record.normal)
        } else {
            unit_direction.refract(&hit_record.normal, ri)
        };
        
        let scattered = Ray::new_time(hit_record.point, direction, ray_in.time);
        Some((Color3::new(1.0, 1.0, 1.0), scattered))
    }
}