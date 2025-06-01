use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Medium {
    boundary: Rc<Hittables>,
    neg_inv_density: f64,
    phase_function: Rc<Materials>,
}

impl Medium {
    // pub fn new(boundary: Rc<Hittables>, density: f64, tex: Rc<Textures>) -> Self {
    //     Self {
    //         boundary,
    //         neg_inv_density: -1./density,
    //         phase_function: Materials::isotropic(tex),
    //     }
    // }

    pub fn solid(boundary: Rc<Hittables>, density: f64, color: Color3) -> Self {
        Self {
            boundary,
            neg_inv_density: -1./density,
            phase_function: Materials::isotropic_solid(color),
        }
    }

    pub fn bounding_box(&self) -> &AABB{
        self.boundary.bounding_box()
    }

    pub fn hit(&self, ray: &Ray, interval: Interval, rng:&mut ThreadRng) -> Option<HitRecord> {
        if let Some(mut hr1) = self.boundary.hit(ray, interval, rng) {
            if let Some(mut hr2) = self.boundary.hit(ray, Interval::new(hr1.t+0.0001, INF), rng) {
                if hr1.t < interval.min {hr1.t = interval.min}
                if hr2.t > interval.max {hr2.t = interval.max}

                if hr1.t >= hr2.t {
                    None
                } else {
                    if hr1.t < 0. {hr1.t = 0.}

                    let ray_length = ray.direction.length();
                    let distance_inside_boundary = (hr2.t - hr1.t) * ray_length;
                    let hit_distance = self.neg_inv_density * gen_01(rng).ln();
                    
                    if hit_distance > distance_inside_boundary {
                        None
                    } else {
                        let t = hr1.t + hit_distance  / ray_length;
                        let point = ray.at(t);
                        let normal = Vec3::new(1., 0., 0.);
                        
                        let u = hr1.u;
                        let v = hr2.v;
                        let mat = self.phase_function.clone();
                        let front_face = true;

                        Some(HitRecord {
                            point,
                            normal,
                            t,
                            u,
                            v,
                            mat,
                            front_face,
                        })
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }


}
