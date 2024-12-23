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

