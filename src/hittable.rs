use crate::*;

pub struct HitRecord {
    pub point : Point3,
    pub normal : Vec3,
    pub t : f64,
    pub front_face : bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, ray : &Ray, outward_normal : &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {*outward_normal} else {-1.0 * *outward_normal}
    }
}

pub trait Hittable {
    fn hit(&self, ray : &Ray, interval : Interval) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center : Point3,
    pub radius : f64,
}

impl Sphere {
    pub fn new(center : Point3, radius : f64) -> Self {
        Sphere {center, radius}
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

        let mut res = HitRecord {point, normal, t, front_face : false};
        res.set_face_normal(ray, &normal);
        Some(res)
    }
}

pub struct HittableList {
    objects : Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn empty() -> Self {
        Self {objects : Vec::new()}
    }

    /* pub fn new(object : Rc<dyn Hittable>) -> Self {
        Self {objects : vec![object]}
    } */

    pub fn add(&mut self, object : Rc<dyn Hittable>) {
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
