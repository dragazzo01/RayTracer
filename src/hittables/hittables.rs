use crate::prelude::*;
use crate::hittables::hit_record::HitRecord;
use crate::hittables::sphere::Sphere;
use crate::hittables::aabb::AABB;
use crate::hittables::bvh::BVHNode;

#[derive(Debug, Copy, Clone)]
pub enum Hittables {
    Sphere(Sphere),
    AABB(AABB),
    BVH(BVHNode),
}

impl Hittables {
    pub fn new_static_sphere(center : Point3, radius : f64, mat : Materials) -> Self {
        Self::Sphere(Sphere::new_static(center, radius, mat))
    }

    pub fn new_moving_sphere(start : Point3, end : Point3, radius : f64, mat : Materials) -> Self {
        Self::Sphere(Sphere::new_moving(start, end, radius, mat))
    }

    pub fn new_node(objects : &Vec<Hittables>, start : usize, end : usize, rng : &mut ThreadRng) -> Self {
        Self::BVH(BVHNode::new(objects, start, end, rng))
    }

    pub fn bounding_box(&self) -> AABB {
        match self {
            Self::Sphere(s) => s.bbox,
            Self::AABB(s) => s,
            Self::BVH(s) => s.bbox,
        }
    }

    pub fn hit(&self, ray : &Ray, interval : Interval) -> Option<HitRecord> {
        match self {
            Self::Sphere(s) => s.hit(ray, interval),
            Self::AABB(s) => panic!("Have a wierd problem where I am returning interval not hit record"),
            Self::BVH(s) => s.hit(ray, interval),
        }
    }
}


#[derive(Debug, Clone)]
pub struct HittableList {
    pub objects : Vec<Hittables>,
    pub bbox : AABB,
}

impl HittableList {
    pub fn empty() -> Self {
        Self {objects : Vec::new()}
    }

    fn add(&mut self, object : Hittables) {
        self.objects.push(object);
        self.bbox = AABB::from_boxes(self.bbox, object.bounding_box())
    }

    pub fn add_static_sphere(&mut self, center : Point3, radius : f64, mat : Materials) {
        self.add(Hittables::new_static_sphere(center, radius, mat));
    }

    pub fn add_moving_sphere(&mut self, start : Point3, end : Point3, radius : f64, mat : Materials) {
        self.add(Hittables::new_moving_sphere(start, end, radius, mat));
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
