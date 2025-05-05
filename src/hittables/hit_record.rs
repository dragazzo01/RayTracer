use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub point : Point3,
    pub normal : Vec3,
    pub t : f64,
    pub mat : Materials,
    pub front_face : bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray : &Ray, outward_normal : &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {*outward_normal} else {-1.0 * *outward_normal}
    }
}