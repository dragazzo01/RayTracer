use crate::prelude::*;
#[derive(Debug, Clone)]
pub struct Translate {
    object: Arc<Hittables>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Arc<Hittables>, offset: Vec3) -> Self {
        let bbox = object.bounding_box().offset(offset);

        Self {
            object,
            offset,
            bbox,
        }
    }

    pub fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    pub fn hit(&self, ray: &Ray, interval: Interval, rng: &mut ThreadRng ) -> Option<HitRecord> {
        let offest_ray = Ray::new_time(ray.origin - self.offset, ray.direction, ray.time);

        match self.object.hit(&offest_ray, interval, rng)  {
            Some(mut hr) => {
                hr.point = hr.point + self.offset;
                Some(hr)
            },
            None => None
        }
    }
}

#[derive(Debug, Clone)]
pub struct RotateY {
    object: Arc<Hittables>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Arc<Hittables>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(INF, INF, INF);
        let mut max = Point3::new(NEG_INF, NEG_INF, NEG_INF);

        for i in (0..2).map(|x| x as f64) {
            for j in (0..2).map(|x| x as f64) {
                for k in (0..2).map(|x| x as f64) {
                    let x = i*bbox.axis_interval(0).max + (1.-i)*bbox.axis_interval(0).min;
                    let y = j*bbox.axis_interval(1).max + (1.-j)*bbox.axis_interval(1).min;
                    let z = k*bbox.axis_interval(2).max + (1.-k)*bbox.axis_interval(2).min;

                    let newx = cos_theta*x + sin_theta*z;
                    let newz = -sin_theta*x + cos_theta*z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bbox = AABB::from_points(min, max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }

    pub fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    pub fn hit(&self, ray: &Ray, interval: Interval, rng: &mut ThreadRng) -> Option<HitRecord> {
        let origin = Point3::new(
            self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z,
            ray.origin.y,
            self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z,
        );

        let direction = Point3::new(
            self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z,
            ray.direction.y,
            self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z,  
        );

        let new_ray = Ray::new_time(origin, direction, ray.time);

        match self.object.hit(&new_ray, interval, rng)  {
            Some(mut hr) => {
                hr.point = Point3::new(
                    self.cos_theta * hr.point.x - self.sin_theta * hr.point.z,
                    hr.point.y,
                    self.sin_theta * hr.point.x + self.cos_theta * hr.point.z,
                );
                
                hr.normal = Point3::new(
                    self.cos_theta * hr.normal.x - self.sin_theta * hr.normal.z,
                    hr.normal.y,
                    self.sin_theta * hr.normal.x + self.cos_theta * hr.normal.z,
                );
                Some(hr)
            },
            None => None
        }
    }
}

