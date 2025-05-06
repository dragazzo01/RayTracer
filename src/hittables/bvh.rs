use crate::prelude::*;
use crate::hittables::hittables::Hittables;
use crate::hittables::aabb::AABB;

#[derive(Debug, Copy, Clone)]
pub struct BVHNode{
    left : Hittables,
    right : Hittables,
    pub bbox : AABB,
}

impl BVHNode {
    pub fn new(objects : &Vec<Hittables>, start : usize, end : usize, rng : &mut ThreadRng) -> Self {
        let axis = gen_int(0, 2, rng);

        let comparator = match axis {
            0 => panic!("unreachable"),
            1 => panic!("unreachable"),
            2 => panic!("unreachable"),
            _ => panic!("unreachable"),
        };

        let span = end - start;
        
        let (left, right) = match span {
            1 => (objects[start], objects[start]),
            2 => (objects[start], objects[start+1]),
            _ => {   
                objects[start..end].sort_by(comparator);
    
                let mid = start + span/2;
    
                (Hittables::new_node(objects, start, mid), Hittables::new_node(objects, mid, start))
            },
        };

        let bbox = AABB::from_boxes(left.bounding_box, right.bounding_box);

        Self {left, right, bbox}

    }

    pub fn from_list(list : &HittableList) -> Self {
        Self::new(&list.objects, 0, list.objects.size)
    }

    pub fn hit(&self, ray : &Ray, ray_t : Interval) -> Option<HitRecord> {
        if self.bbox.hit(ray, ray_t).is_none() {return None;};

        let hit_left = self.left.hit(ray, ray_t);
        let new_t = match hit_left {
            None => ray_t,
            Some(hr) => Interval::new(ray_t.min, hr.t)
        };
        let hit_right = self.right.hit(ray, new_t);

        match hit_right {
            None => hit_left,
            Some(hr) => hit_right
        }
    }
}