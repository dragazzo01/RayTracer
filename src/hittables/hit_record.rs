use crate::prelude::*;

/// Represents the record of a ray hitting a surface.
///
/// # Fields
/// - `point`: The point of intersection.
/// - `normal`: The normal vector at the intersection point.
/// - `t`: The ray parameter at the intersection.
/// - `mat`: The material of the surface hit.
/// - `front_face`: A boolean indicating if the ray hit the front face of the surface.
#[derive(Debug, Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub mat: Materials,
    pub front_face: bool,
}

impl HitRecord {
    /// Sets the face normal of the hit record based on the ray direction and outward normal.
    ///
    /// # Arguments
    /// - `ray`: The ray that hit the surface.
    /// - `outward_normal`: The outward normal vector at the intersection point.
    ///
    /// # Details
    /// This method determines whether the ray hit the front face or the back face of the surface
    /// and adjusts the normal direction accordingly.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -1.0 * *outward_normal
        }
    }
}
