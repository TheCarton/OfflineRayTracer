use crate::aabb::AABB;
use crate::{Ray};

pub struct BVH {
    boxes: Vec<AABB>
}

impl BVH {
    pub fn check_ray(&self, r: Ray, t_min:f32, t_max:f32) -> bool { // Bounding Volume Requirement
        for bounding_box in &self.boxes {
            if bounding_box.hit_box(r, t_min, t_max) {
                return true;
            }
        }
        return false;
    }

    pub fn new() -> BVH {
        BVH {
            boxes: Vec::new(),
        }
    }

    pub fn add(&mut self, b: AABB) {
        self.boxes.push(b);
    }


}