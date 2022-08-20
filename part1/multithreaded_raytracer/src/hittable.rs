use rand::prelude::ThreadRng;

use crate::{dot, Point, Ray};
use crate::aabb::AABB;
use crate::scatter_results::ScatterResults;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Point,
    pub t: f32,
    pub front_face: bool,
    pub scatter_results: Option<ScatterResults>,
}

pub trait Hittable : Send {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rng: &mut ThreadRng) -> Option<HitRecord>;
    fn scatter(&self, r_in: Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<ScatterResults>;
    fn get_bounding_box(&self) -> Option<AABB>;
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Point) {
        self.front_face = dot(r.direction, outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}