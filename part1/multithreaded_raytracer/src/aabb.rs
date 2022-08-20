use crate::{Point, Ray};

pub struct AABB {
    pub min: Point,
    pub max: Point,
}

impl AABB {
    pub fn hit_box(&self, r: Ray, t_min: f32, t_max: f32) -> bool { // Bounding Volume Requirement
        let mut t_min_aabb= t_min;
        let mut t_max_aabb = t_max;
        for a in 0..3 {
            let t0a = (self.min[a] - r.origin[a]) / r.direction[a];
            let t0b = (self.max[a] - r.origin[a]) / r.direction[a];
            let t0 = t0a.min(t0b);

            let t1a = (self.min[a] - r.origin[a]) / r.direction[a];
            let t1b = (self.max[a] - r.origin[a]) / r.direction[a];
            let t1 = t1a.max(t1b);

            t_min_aabb = t0.max(t_min_aabb);
            t_max_aabb = t1.min(t_max_aabb);
            if t_max_aabb <= t_min_aabb { return false; }
        }
        return true;
    }
}