use std::cmp::Ordering;

use crate::prelude::*;
use crate::hittables::hittables::{Hittables, HittableList};
use crate::hittables::aabb::AABB;
//use std::

#[derive(Debug, Clone)]
pub enum BVHNode {
    Leaf(Hittables),
    Node {
        left : Box<BVHNode>,
        right : Box<BVHNode>,
        bbox : AABB,
    }, 
    
}

impl BVHNode {
    pub fn new(objects : &mut Vec<Hittables>, start : usize, end : usize, rng : &mut ThreadRng) -> Self {

        println!("Start {:?}, End {:?}", start, end);
        let span = end - start;
        
        if span == 1 {return Self::Leaf(objects[start]);}
            // 2 => {
            //     let left = objects[start];
            //     let right = objects[start+1];

            //     let bbox = AABB::from_boxes(left.bounding_box(), right.bounding_box());
                
            //     let left = Box::new(Self::Leaf(left, left.bounding_box()));
            //     let right = Box::new(Self::Leaf(right, right.bounding_box()));

                
            //     Self::Node {left, right, bbox}
            // },
   
        let axis = gen_int(0, 2, rng);

        let comparator = match axis {
            0 => Self::box_compare_x,
            1 => Self::box_compare_y,
            2 => Self::box_compare_z,
            _ => panic!("unreachable"),
        };

        objects[start..end].sort_by(comparator);
    
        let mid = start + span/2;
    
        let left = Box::new(Self::new(objects, start, mid, rng));
        let right = Box::new(Self::new(objects, mid, end, rng));
        let bbox = AABB::from_boxes(left.bounding_box(), right.bounding_box());
        Self::Node { left, right, bbox }
    }


    pub fn from_list(list : &mut HittableList, rng : &mut ThreadRng) -> Self {
        let len = list.objects.len();
        println!("Total Objects: {len}");
        //println!("List: {:?}", list);
        Self::new(&mut list.objects, 0, len, rng)
    }

    fn box_compare(a : &Hittables, b : &Hittables, axis : i32) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis);
        let b_axis_interval = b.bounding_box().axis_interval(axis);
        
        if a_axis_interval.min < b_axis_interval.min {Ordering::Less}
        else {Ordering::Greater}
    }

    fn box_compare_x(a : &Hittables, b : &Hittables) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_compare_y(a : &Hittables, b : &Hittables) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_compare_z(a : &Hittables, b : &Hittables) -> Ordering {
        Self::box_compare(a, b, 2)
    }

    pub fn bounding_box(&self) -> AABB {
        match self {
            Self::Leaf(x) => x.bounding_box(),
            Self::Node {left : _, right : _, bbox } => *bbox,
        }
    }

    pub fn hit(&self, ray : &Ray, ray_t : Interval) -> Option<HitRecord> {
        if self.bounding_box().hit(ray).is_none() {return None;};

        match self {
            Self::Leaf(object) => object.hit(ray, ray_t),
            Self::Node{left, right, ..} => {
                let mut final_hit_record = None;
                let mut closest_so_far = ray_t.max;

                
                match left.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                    Some(hr) => {
                        closest_so_far = hr.t;
                        final_hit_record = Some(hr);
                    },
                    None => (),
                }

                match right.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                    Some(hr) => {
                        final_hit_record = Some(hr);
                    },
                    None => (),
                }

                final_hit_record
            }
        }
        
    }
}