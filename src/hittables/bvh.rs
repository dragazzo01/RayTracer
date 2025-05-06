use std::cmp::Ordering;

use crate::hittables::aabb::AABB;
use crate::hittables::hittables::{HittableList, Hittables};
use crate::prelude::*;

/// Represents a Bounding Volume Hierarchy (BVH) node.
///
/// A BVH is used to accelerate ray tracing by organizing objects into a tree structure.
///
/// # Variants
/// - `Leaf`: A leaf node containing a single hittable object.
/// - `Node`: An internal node containing two child nodes and a bounding box.
#[derive(Debug, Clone)]
pub enum BVHNode {
    Leaf(Hittables),
    Node {
        left: Box<BVHNode>,
        right: Box<BVHNode>,
        bbox: AABB,
    },
}

impl BVHNode {
    /// Constructs a new BVH node from a list of hittable objects.
    ///
    /// # Arguments
    /// - `objects`: A mutable reference to a vector of hittable objects.
    /// - `start`: The starting index of the objects to include in this node.
    /// - `end`: The ending index (exclusive) of the objects to include in this node.
    ///
    /// # Returns
    /// A new `BVHNode` instance.
    fn new(objects: &mut Vec<Hittables>, start: usize, end: usize) -> Self {
        let span = end - start;

        if span == 1 {
            return Self::Leaf(objects[start]);
        }

        let mut bbox = AABB::empty();
        for obj in objects.iter() {
            bbox = AABB::from_boxes(bbox, obj.bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_compare_x,
            1 => Self::box_compare_y,
            2 => Self::box_compare_z,
            _ => panic!("unreachable"),
        };

        objects[start..end].sort_by(comparator);

        let mid = start + span / 2;

        let left = Box::new(Self::new(objects, start, mid));
        let right = Box::new(Self::new(objects, mid, end));

        Self::Node { left, right, bbox }
    }

    /// Constructs a BVH tree from a `HittableList`.
    ///
    /// # Arguments
    /// - `list`: A mutable reference to a `HittableList` containing the objects to organize.
    ///
    /// # Returns
    /// The root node of the constructed BVH tree.
    pub fn from_list(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        Self::new(&mut list.objects, 0, len)
    }

    /// Compares two hittable objects along a specified axis.
    ///
    /// # Arguments
    /// - `a`: The first hittable object.
    /// - `b`: The second hittable object.
    /// - `axis`: The axis to compare (0 for x, 1 for y, 2 for z).
    ///
    /// # Returns
    /// An `Ordering` indicating the relative positions of the objects along the axis.
    fn box_compare(a: &Hittables, b: &Hittables, axis: i32) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis);
        let b_axis_interval = b.bounding_box().axis_interval(axis);

        if a_axis_interval.min < b_axis_interval.min {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    /// Compares two hittable objects along the x-axis.
    ///
    /// # Arguments
    /// - `a`: The first hittable object.
    /// - `b`: The second hittable object.
    ///
    /// # Returns
    /// An `Ordering` indicating the relative positions of the objects along the x-axis.
    fn box_compare_x(a: &Hittables, b: &Hittables) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    /// Compares two hittable objects along the y-axis.
    ///
    /// # Arguments
    /// - `a`: The first hittable object.
    /// - `b`: The second hittable object.
    ///
    /// # Returns
    /// An `Ordering` indicating the relative positions of the objects along the y-axis.
    fn box_compare_y(a: &Hittables, b: &Hittables) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    /// Compares two hittable objects along the z-axis.
    ///
    /// # Arguments
    /// - `a`: The first hittable object.
    /// - `b`: The second hittable object.
    ///
    /// # Returns
    /// An `Ordering` indicating the relative positions of the objects along the z-axis.
    fn box_compare_z(a: &Hittables, b: &Hittables) -> Ordering {
        Self::box_compare(a, b, 2)
    }

    /// Returns the bounding box of the BVH node.
    ///
    /// # Returns
    /// The `AABB` representing the bounding box of this node.
    pub fn bounding_box(&self) -> AABB {
        match self {
            Self::Leaf(x) => x.bounding_box(),
            Self::Node {
                left: _,
                right: _,
                bbox,
            } => *bbox,
        }
    }

    /// Determines if a ray intersects the BVH node.
    ///
    /// # Arguments
    /// - `ray`: The ray to test for intersection.
    /// - `ray_t`: The valid interval for the ray parameter `t`.
    ///
    /// # Returns
    /// An `Option<HitRecord>` containing the hit information if the ray intersects an object
    /// in the BVH node, or `None` if there is no intersection.
    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        self.bounding_box().hit(ray)?;
        
        match self {
            Self::Leaf(object) => object.hit(ray, ray_t),
            Self::Node { left, right, .. } => {
                let mut final_hit_record = None;
                let mut closest_so_far = ray_t.max;

                if let Some(hr) = left.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                    closest_so_far = hr.t;
                    final_hit_record = Some(hr);

                }

                if let Some(hr) = right.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                    final_hit_record = Some(hr);
                }

                final_hit_record
            }
        }
    }
}
