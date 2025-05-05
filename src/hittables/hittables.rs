use crate::prelude::*;
use crate::hittables::hit_record::HitRecord;
use crate::hittables::sphere::Sphere;

#[derive(Debug, Copy, Clone)]
pub enum Hittables {
    Sphere(Sphere),
}

impl Hittables {
    pub fn new_sphere(center : Point3, radius : f64, mat : Materials) -> Self {
        Self::Sphere(Sphere::new_static(center, radius, mat))
    }

    pub fn hit(&self, ray : &Ray, interval : Interval) -> Option<HitRecord> {
        match self {
            Self::Sphere(s) => s.hit(ray, interval),
        }
    }
}


#[derive(Debug, Clone)]
pub struct HittableList {
    objects : Vec<Hittables>,
}

impl HittableList {
    pub fn empty() -> Self {
        Self {objects : Vec::new()}
    }

    fn add(&mut self, object : Hittables) {
        self.objects.push(object);
    }

    pub fn add_sphere(&mut self, center : Point3, radius : f64, mat : Materials) {
        self.add(Hittables::new_sphere(center, radius, mat));
    }

    pub fn hit(&self, ray : &Ray, interval : Interval) -> Option<HitRecord> {
        let mut final_hit_record = None;
        let mut closest_so_far = interval.max;

        for object in &self.objects {
            match object.hit(ray, Interval::new(interval.min, closest_so_far)) {
                Some(hr) => {
                        closest_so_far = hr.t;
                        final_hit_record = Some(hr);
                        },
                None => (),
                
            }
        }

        return final_hit_record;
    }
}
