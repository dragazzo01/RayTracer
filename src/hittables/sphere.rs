use crate::hittables::aabb::AABB;
use crate::prelude::*;

/// Represents a sphere that can be static or moving in the scene.
///
/// # Fields
/// - `center`: The center of the sphere, represented as a `Ray`.
/// - `radius`: The radius of the sphere.
/// - `mat`: The material of the sphere.
/// - `bbox`: The bounding box of the sphere.
#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub mat: Materials,
    pub bbox: AABB,
}

impl Sphere {
    /// Creates a new static sphere.
    ///
    /// # Arguments
    /// - `center`: The center of the sphere as a `Point3`.
    /// - `radius`: The radius of the sphere.
    /// - `mat`: The material of the sphere.
    ///
    /// # Returns
    /// A new `Sphere` instance.
    pub fn new_static(center: Point3, radius: f64, mat: Materials) -> Self {
        let rvec = Vec3::new(radius, radius, radius);

        Self {
            center: Ray::new(center, Vec3::zero()),
            radius,
            mat,
            bbox: AABB::from_points(center - rvec, center + rvec),
        }
    }

    /// Creates a new moving sphere.
    ///
    /// # Arguments
    /// - `center_start`: The starting center position of the sphere as a `Point3`.
    /// - `center_end`: The ending center position of the sphere as a `Point3`.
    /// - `radius`: The radius of the sphere.
    /// - `mat`: The material of the sphere.
    ///
    /// # Returns
    /// A new `Sphere` instance.
    #[allow(dead_code)]
    pub fn new_moving(
        center_start: Point3,
        center_end: Point3,
        radius: f64,
        mat: Materials,
    ) -> Self {
        let center = Ray::new(center_start, center_end - center_start);

        let rvec = Vec3::new(radius, radius, radius);
        let box1 = AABB::from_points(center.at(0.) - rvec, center.at(0.) + rvec);
        let box2 = AABB::from_points(center.at(1.) - rvec, center.at(1.) + rvec);
        Self {
            center,
            radius,
            mat,
            bbox: AABB::from_boxes(&box1, &box2),
        }
    }

    pub fn get_sphere_uv(point : &Point3) -> (f64, f64) {
        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + PI;

        (phi / (2.*PI), theta / PI)
    }

    /// Determines if a ray hits the sphere.
    ///
    /// # Arguments
    /// - `ray`: The ray to test for intersection.
    /// - `interval`: The valid interval for the ray parameter `t`.
    ///
    /// # Returns
    /// An `Option<HitRecord>` containing the hit information if the ray intersects the sphere,
    /// or `None` if there is no intersection.
    pub fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        //determine if ray hits sphere
        let center = self.center.at(ray.time);
        let oc = center - ray.origin;
        let a = ray.direction.norm();
        let h = ray.direction.dot(&oc);
        let c = oc.norm() - self.radius * self.radius;
        let discriminant = h * h - a * c;
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

        let (u, v) = Self::get_sphere_uv(&normal);

        let mut res = HitRecord {
            point,
            normal,
            t,
            mat: self.mat.clone(),
            front_face: false,
            u,
            v,
        };
        res.set_face_normal(ray, &normal);
        Some(res)
    }
}
