use crate::hittables::aabb::AABB;
use crate::hittables::hit_record::HitRecord;
use crate::hittables::sphere::Sphere;
use crate::prelude::*;

/// Represents a collection of hittable objects in the scene.
///
/// # Variants
/// - `Sphere`: A hittable sphere.
#[derive(Debug, Clone)]
pub enum Hittables {
    Sphere(Sphere),
}

impl Hittables {
    /// Creates a new static sphere.
    ///
    /// # Arguments
    /// - `center`: The center of the sphere as a `Point3`.
    /// - `radius`: The radius of the sphere.
    /// - `mat`: The material of the sphere.
    ///
    /// # Returns
    /// A new `Hittables` instance containing the static sphere.
    pub fn new_static_sphere(center: Point3, radius: f64, mat: Arc<Materials>) -> Self {
        Self::Sphere(Sphere::new_static(center, radius, mat))
    }

    /// Creates a new moving sphere.
    ///
    /// # Arguments
    /// - `start`: The starting center position of the sphere as a `Point3`.
    /// - `end`: The ending center position of the sphere as a `Point3`.
    /// - `radius`: The radius of the sphere.
    /// - `mat`: The material of the sphere.
    ///
    /// # Returns
    /// A new `Hittables` instance containing the moving sphere.
    pub fn new_moving_sphere(start: Point3, end: Point3, radius: f64, mat: Arc<Materials>) -> Self {
        Self::Sphere(Sphere::new_moving(start, end, radius, mat))
    }

    /// Returns the bounding box of the hittable object.
    ///
    /// # Returns
    /// An `AABB` representing the bounding box of the object.
    pub fn bounding_box(&self) -> &AABB {
        match self {
            Self::Sphere(s) => &s.bbox,
        }
    }

    /// Determines if a ray hits the hittable object.
    ///
    /// # Arguments
    /// - `ray`: The ray to test for intersection.
    /// - `interval`: The valid interval for the ray parameter `t`.
    ///
    /// # Returns
    /// An `Option<HitRecord>` containing the hit information if the ray intersects the object,
    /// or `None` if there is no intersection.
    pub fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        match self {
            Self::Sphere(s) => s.hit(ray, interval),
        }
    }
}

/// Represents a list of hittable objects in the scene.
///
/// # Fields
/// - `objects`: A vector of hittable objects.
/// - `bbox`: The bounding box enclosing all objects in the list.
#[derive(Debug, Clone)]
pub struct HittableList {
    pub objects: Vec<Hittables>,
    pub bbox: AABB,
}

impl HittableList {
    /// Creates an empty hittable list.
    ///
    /// # Returns
    /// A new `HittableList` instance with no objects.
    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::universe(),
        }
    }

    fn add(&mut self, object: Hittables) {
        self.bbox = AABB::from_boxes(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }

    /// Adds a static sphere to the hittable list.
    ///
    /// # Arguments
    /// - `center`: The center of the sphere as a `Point3`.
    /// - `radius`: The radius of the sphere.
    /// - `mat`: The material of the sphere.
    pub fn add_static_sphere(&mut self, center: Point3, radius: f64, mat: Arc<Materials>) {
        self.add(Hittables::new_static_sphere(center, radius, mat));
    }

    /// Adds a moving sphere to the hittable list.
    ///
    /// # Arguments
    /// - `start`: The starting center position of the sphere as a `Point3`.
    /// - `end`: The ending center position of the sphere as a `Point3`.
    /// - `radius`: The radius of the sphere.
    /// - `mat`: The material of the sphere.
    #[allow(dead_code)]
    pub fn add_moving_sphere(&mut self, start: Point3, end: Point3, radius: f64, mat: Arc<Materials>) {
        self.add(Hittables::new_moving_sphere(start, end, radius, mat));
    }

    /// Determines if a ray hits any object in the hittable list.
    ///
    /// # Arguments
    /// - `ray`: The ray to test for intersection.
    /// - `interval`: The valid interval for the ray parameter `t`.
    ///
    /// # Returns
    /// An `Option<HitRecord>` containing the hit information if the ray intersects any object,
    /// or `None` if there is no intersection.
    #[allow(dead_code)]
    pub fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut final_hit_record = None;
        let mut closest_so_far = interval.max;

        for object in &self.objects {
            if let Some(hr) =  object.hit(ray, Interval::new(interval.min, closest_so_far)) {
                closest_so_far = hr.t;
                final_hit_record = Some(hr);
            }   
        }

        final_hit_record
    }
}
