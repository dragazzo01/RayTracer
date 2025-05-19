use crate::prelude::*;
use crate::hittables::aabb::AABB;

#[derive(Debug, Clone)]
pub struct Quad {
    q : Point3, //bottom corner
    u : Vec3, //one basis vector (q + u is one more corner)
    v : Vec3, //second basis vector for plane (q + v is another corner)
    w : Vec3, //used for figureing out where ray intersects plane
    normal : Vec3, 
    d : f64,
    mat : Materials,
    bbox : AABB,
}

impl Quad {
    pub fn new(q : Point3, u : Vec3, v : Vec3, mat : Materials) -> Self {
        let diag1 = AABB::from_points(q, q+u+v);
        let diag2 = AABB::from_points(q+u, q+v);
        let mut bbox = AABB::from_boxes(&diag1, &diag2);
        bbox.pad_to_minimums();

        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&q);

        let w = n / (n.dot(&n));

        Self {
            q, 
            u, 
            v,
            w,
            normal,
            d, 
            mat,
            bbox,
        }
    }

    pub fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    pub fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction);
        if denom.abs() < 1e-8 {return None}

        let t = (self.d - (self.normal.dot(&ray.origin))) / denom;
        if !interval.contains(t) {return None}
        

        let intersection = ray.at(t);
        let planar_space_intersection = intersection - self.q;
        let alpha = self.w.dot(&planar_space_intersection.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_space_intersection));

        
        if !Self::is_interior(alpha, beta) {return None}
        
        
        let mut res = HitRecord {
            point : intersection,
            normal : self.normal,
            t,
            mat: self.mat.clone(),
            front_face: false,
            u : alpha,
            v: beta,
        };
        res.set_face_normal(ray, &self.normal);
        Some(res)
    }

    fn is_interior(alpha : f64, beta : f64) -> bool {
        let unit = Interval::new(0.,1.);

        return unit.contains(alpha) && unit.contains(beta)
    }
}