use crate::hittables::sphere::Sphere;
use crate::hittables::quad::Quad;
use crate::hittables::bvh::BVHNode;
use crate::hittables::translation::Translate;
use crate::prelude::*;

use super::translation::RotateY;

/// Represents a collection of hittable objects in the scene.
///
/// # Variants
/// - `Sphere`: A hittable sphere.
#[derive(Debug, Clone)]
pub enum Hittables {
    Sphere(Sphere),
    BVH(Box<BVHNode>),
    Quad(Quad),
    Translate(Translate),
    RotY(RotateY),
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
    pub fn new_static_sphere(center: Point3, radius: f64, mat: Arc<Materials>) -> Arc<Self> {
        Arc::new(Self::Sphere(Sphere::new_static(center, radius, mat)))
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
    pub fn new_moving_sphere(start: Point3, end: Point3, radius: f64, mat: Arc<Materials>) -> Arc<Self> {
        Arc::new(Self::Sphere(Sphere::new_moving(start, end, radius, mat)))
    }

    pub fn new_quad(q : Point3, u : Vec3, v : Vec3, mat : Arc<Materials>) -> Arc<Self> {
        Arc::new(Self::Quad(Quad::new(q, u, v, mat)))
    }

    pub fn translate(object: Arc<Self>, offset: Vec3) -> Arc<Self> {
        Arc::new(Self::Translate(Translate::new(object, offset)))
    }

    pub fn rotate_y(object: Arc<Self>, degree: f64) -> Arc<Self> {
        Arc::new(Self::RotY(RotateY::new(object, degree)))
    }

    /// Returns the bounding box of the hittable object.
    ///
    /// # Returns
    /// An `AABB` representing the bounding box of the object.
    pub fn bounding_box(&self) -> &AABB {
        match self {
            Self::Sphere(obj) => obj.bounding_box(),
            Self::BVH(obj) => obj.bounding_box(),
            Self::Quad(obj) => obj.bounding_box(),
            Self::Translate(obj) => obj.bounding_box(),
            Self::RotY(obj) => obj.bounding_box(),
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
            Self::Sphere(obj) => obj.hit(ray, interval),
            Self::BVH(obj) => obj.hit(ray, interval),
            Self::Quad(obj) => obj.hit(ray, interval),
            Self::Translate(obj) => obj.hit(ray, interval),
            Self::RotY(obj) => obj.hit(ray, interval),
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
    pub objects: Vec<Arc<Hittables>>,
}

impl HittableList {
    /// Creates an empty hittable list.
    ///
    /// # Returns
    /// A new `HittableList` instance with no objects.
    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    fn add(&mut self, object: Arc<Hittables>) {
        self.objects.push(object);
    }

    pub fn append(&mut self, new:&mut HittableList) {
        self.objects.append(&mut new.objects);
    }

    pub fn translate(&mut self, offset: Vec3) {
        self.objects = self.objects
            .drain(..)
            .map(|obj| Hittables::translate(
                obj.clone(),
                offset,
            ))
            .collect();
    }

    pub fn rotate_y(&mut self, degree: f64) {
        self.objects = self.objects
            .drain(..)
            .map(|obj| Hittables::rotate_y(
                obj.clone(),
                degree,
            ))
            .collect();
    }

    /// Adds a static sphere to the hittable list.
    ///
    /// # Arguments
    /// - `center`: The center of the sphere as a `Point3`.
    /// - `radius`: The radius of the sphere.
    /// - `mat`: The material of the sphere.
    pub fn add_sphere(&mut self, center: Point3, radius: f64, mat: Arc<Materials>) {
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

    pub fn add_quad(&mut self, q : Point3, u : Vec3, v : Vec3, mat : Arc<Materials>) {
        self.add(Hittables::new_quad(q, u, v, mat))
    }

    pub fn create_box(a : Point3, b : Point3, mat : Arc<Materials>) -> Self {
        Quad::create_box(a, b, mat)
    }

    pub fn create_bvh(&mut self) -> Hittables {
        Hittables::BVH(Box::new(BVHNode::from_list(self)))
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
