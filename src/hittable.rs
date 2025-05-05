use crate::prelude::*;

pub struct HitRecord {
    pub point : Point3,
    pub normal : Vec3,
    pub t : f64,
    pub mat : Materials,
    pub front_face : bool,
}

#[derive(Debug, Copy, Clone)]
pub enum Hittables {
    Sphere(Sphere),
}

impl Hittable for Hittables {
    fn hit(&self, ray : &Ray, interval : Interval) -> Option<HitRecord> {
        match self {
            Hittables::Sphere(s) => s.hit(ray, interval),
        }
    }
}

impl HitRecord {
    fn set_face_normal(&mut self, ray : &Ray, outward_normal : &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {*outward_normal} else {-1.0 * *outward_normal}
    }
}

pub trait Hittable : Send + Sync {
    fn hit(&self, ray : &Ray, interval : Interval) -> Option<HitRecord>;
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center : Point3,
    pub radius : f64,
    mat : Materials,
    
}

impl Sphere {
    pub fn new(center : Point3, radius : f64, mat : Materials) -> Hittables {
        Hittables::Sphere(Self {center, radius, mat})
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray : &Ray, interval : Interval) -> Option<HitRecord> {
        //determine if ray hits sphere 
        let oc = self.center - ray.origin;
        let a = ray.direction.norm();
        let h = ray.direction.dot(&oc);
        let c = oc.norm() - self.radius*self.radius;
        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return None;
        } 

        //Find nearest root that lies in range
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;

        let mut res = HitRecord {point, normal, t, mat : self.mat, front_face : false};
        res.set_face_normal(ray, &normal);
        Some(res)
    }
}

//#[derive(Clone)]
#[derive(Debug, Clone)]
pub struct HittableList {
    objects : Vec<Hittables>,
}

impl HittableList {
    pub fn empty() -> Self {
        Self {objects : Vec::new()}
    }

    /* pub fn new(object : Arc<dyn Hittable>) -> Self {
        Self {objects : vec![object]}
    } */

    pub fn add(&mut self, object : Hittables) {
        self.objects.push(object);
    }

    /* pub fn clear(&mut self) {
        self.objects.clear()
    } */


}
impl Hittable for HittableList {
    fn hit(&self, ray : &Ray, interval : Interval) -> Option<HitRecord> {
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
