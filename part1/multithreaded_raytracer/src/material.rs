use rand::prelude::ThreadRng;
use crate::{Point, random_unit_vector, Ray};
use crate::hittable::HitRecord;

pub trait Material {
    fn scatter(&self, normal: Point, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Point)>;
}

pub struct Lambertian{
    pub albedo: Point
}

impl Material for Lambertian {
    fn scatter(&self, normal: Point, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Ray, Point)> {
        let mut scatter_direction = normal + random_unit_vector(rng);
        if scatter_direction.near_zero() {
            scatter_direction = normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

