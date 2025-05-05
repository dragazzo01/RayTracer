use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center : Ray,
    pub radius : f64,
    mat : Materials,
    
}

impl Sphere {
    //static Shere
    pub fn new_static(center : Point3, radius : f64, mat : Materials) -> Self {
        Self {center : Ray::new(center, Vec3::zero()), radius, mat}
    }


    #[allow(dead_code)]
    pub fn new_moving(center_start : Point3, center_end : Point3, radius : f64, mat : Materials) -> Self {
        Self {
            center : Ray::new(center_start, center_end - center_start),
            radius, 
            mat
        }
    }

    pub fn hit(&self, ray : &Ray, interval : Interval) -> Option<HitRecord> {
        //determine if ray hits sphere
        let center = self.center.at(ray.time);
        let oc = center - ray.origin;
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
        let normal = (point - center) / self.radius;

        let mut res = HitRecord {point, normal, t, mat : self.mat, front_face : false};
        res.set_face_normal(ray, &normal);
        Some(res)
    }
}