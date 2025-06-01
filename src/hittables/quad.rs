use crate::prelude::*;
use crate::hittables::hittables::HittableList;

#[derive(Debug, Clone)]
pub struct Quad {
    q : Point3, //bottom corner
    u : Vec3, //one basis vector (q + u is one more corner)
    v : Vec3, //second basis vector for plane (q + v is another corner)
    w : Vec3, //used for figureing out where ray intersects plane
    normal : Vec3, 
    d : f64,
    mat : Rc<Materials>,
    bbox : AABB,
}

impl Quad {
    pub fn new(q : Point3, u : Vec3, v : Vec3, mat : Rc<Materials>) -> Self {
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

    pub fn create_box(a : Point3, b : Point3, mat: Rc<Materials>) -> HittableList {
        let mut sides: HittableList = HittableList::empty();

        let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

        let dx = Vec3::new(max.x - min.x, 0., 0.);
        let dy = Vec3::new(0., max.y - min.y, 0.);
        let dz = Vec3::new(0., 0., max.z - min.z);

        sides.add_quad(Point3::new(min.x, min.y, max.z),  dx,  dy, mat.clone()); // front
        sides.add_quad(Point3::new(max.x, min.y, max.z), -1.*dz,  dy, mat.clone()); // right
        sides.add_quad(Point3::new(max.x, min.y, min.z), -1.*dx,  dy, mat.clone()); // back
        sides.add_quad(Point3::new(min.x, min.y, min.z),  dz,  dy, mat.clone()); // left
        sides.add_quad(Point3::new(min.x, max.y, max.z),  dx, -1.*dz, mat.clone()); // top
        sides.add_quad(Point3::new(min.x, min.y, min.z),  dx,  dz, mat); // bottom

        sides
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
            mat : self.mat.clone(),
            front_face: false,
            u : alpha,
            v: beta,
        };
        res.set_face_normal(ray, &self.normal);
        Some(res)
    }

    //THIS IS ONLY THING YOU CHANGE FOR TRIANGLES/ANYOTHER PLANER THING
    fn is_interior(alpha : f64, beta : f64) -> bool {
        let unit = Interval::new(0.,1.);

        return unit.contains(alpha) && unit.contains(beta)
    }
}
