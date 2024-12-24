use crate::*;

pub trait Material {
    fn scatter(&self, ray : &Ray, hit_record : &HitRecord) -> Option<(Color3, Ray)>;
}

#[derive(Clone)]
pub struct Matte {
    _x : i32, 
}

impl Material for Matte {
    fn scatter(&self, _ray : &Ray, _hit_record : &HitRecord) -> Option<(Color3, Ray)> {
        None
    }
}

impl Matte {
    pub fn new() -> Self {
        Self {_x : 0}
    }
}

pub struct Lambertian {
    albedo : Color3,
}

impl Lambertian {
    pub fn new(albedo : Color3) -> Self {
        Self {albedo}
    }
}

impl Material for Lambertian {
    fn scatter (&self, _ray : &Ray, hit_record : &HitRecord) -> Option<(Color3, Ray)> {
        let scatter_direction = {
            let res = hit_record.normal + Vec3::random_unit();
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

pub struct Metal {
    albedo : Color3,
    fuzz : f64,
}

impl Metal {
    pub fn new(albedo : Color3, fuzz : f64) -> Self {
        let fuzz = if fuzz < 1.0 {fuzz} else {1.0};
        Self {albedo, fuzz}
    }
}

impl Material for Metal {
    fn scatter (&self, ray_in : &Ray, hit_record : &HitRecord) -> Option<(Color3, Ray)> {
        let reflected = 
            ray_in.direction.reflect(&hit_record.normal).normalize() 
            + self.fuzz * Vec3::random_unit();
        let scattered = Ray::new(hit_record.point, reflected);


        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
        
    }
}

pub struct Dielectric {
    refraction_index : f64,
}

impl Dielectric {
    pub fn new(refraction_index : f64) -> Self {
        Self {refraction_index}
    }
}

fn reflectance(cosine : f64, refraction_index : f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0)*f64::powf(1.0 - cosine, 5.0)
}

impl Material for Dielectric {
    fn scatter (&self, ray_in : &Ray, hit_record : &HitRecord) -> Option<(Color3, Ray)> {
        let ri = if hit_record.front_face {1.0 / self.refraction_index} else {self.refraction_index};
        let unit_direction = ray_in.direction.normalize();
    
        let cos_theta = f64::min(-1.0 * unit_direction.dot(&hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let direction = 
        if ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > random_01() {
            unit_direction.reflect(&hit_record.normal)
        } else {
            unit_direction.refract(&hit_record.normal, ri)
        };
        
        let scattered = Ray::new(hit_record.point, direction);
        Some((Color3::new(1.0, 1.0, 1.0), scattered))
    }
}



