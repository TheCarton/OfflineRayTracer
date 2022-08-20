use rand::rngs::ThreadRng;

use crate::bvh::BVH;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList
{
    hittable_list: Vec<Box<dyn Hittable + Send + Sync>>,
    pub bvh: BVH,
}

pub trait CheckHits : Send {
    fn get_hits(&self, r: Ray, t_min: f32, t_max: f32, rng: &mut ThreadRng) -> Option<HitRecord>;
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            hittable_list: Vec::new(),
            bvh: BVH::new(),
        }
    }

    pub fn add<U: Hittable + 'static + Send + Sync>(&mut self, o: U) {
        if let Some(bounding_box) = o.get_bounding_box() {
            self.bvh.add(bounding_box);
        }
        self.hittable_list.push(Box::new(o));
    }
}

impl CheckHits for HittableList {
    fn get_hits(&self, r: Ray, t_min: f32, t_max: f32, rng: &mut ThreadRng) -> Option<HitRecord> {
        let mut _hit_anything = false;
        let mut _closest_so_far = t_max;
        for object in &self.hittable_list {
            if let Some(mut rec) = object.hit(r, t_min, t_max, rng) {
                _hit_anything = true;
                _closest_so_far = rec.t.clone();
                rec.scatter_results = object.scatter(r, &rec, rng);
                return Some(rec);
            }
        }
        return None;
    }
}