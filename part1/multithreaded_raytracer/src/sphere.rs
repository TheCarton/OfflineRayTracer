use num::pow;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::{Point, random_unit_vector, Ray};
use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::scatter_results::ScatterResults;
use crate::utility::{dot, random_in_unit_sphere, reflect, refract, unit_vector};

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}


fn hit_sphere<T: Into<Sphere> + Hittable + Clone>(sphere: T, r: Ray, t_min: f32, t_max: f32, rng: &mut ThreadRng) -> Option<HitRecord>
    where Sphere: From<T>
{
    let s: Sphere = sphere.clone().into();
    let oc = r.origin - s.center;
    let a = r.direction.length_squared();
    let half_b = dot(oc, r.direction);
    let c = oc.length_squared() - s.radius * s.radius;

    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return None;
    }
    let sqrtd = discriminant.sqrt();

    let mut root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
        root = (-half_b + sqrtd) / a;
        if root < t_min || t_max < root {
            return None;
        }
    }


    let t = root;
    let p = r.at(root);
    let outward_normal: Point = (p - s.center) / s.radius;
    let temp_normal = Point::default();

    let mut rec = HitRecord {
        p,
        normal: temp_normal,
        t,
        front_face: true,
        scatter_results: None,
    };
    rec.scatter_results = sphere.scatter(r, &rec, rng);
    rec.set_face_normal(r, outward_normal);
    Some(rec)
}

fn sphere_to_bounding_box<T: Into<Sphere>>(sphere: T) -> Option<AABB> {
    let sphere = sphere.into();
    Some(AABB {
        min: sphere.center - Point::new(sphere.radius, sphere.radius, sphere.radius),
        max: sphere.center + Point::new(sphere.radius, sphere.radius, sphere.radius),
    })
}

impl From<LambertianSphere> for Sphere {
    fn from(lam: LambertianSphere) -> Self {
        Sphere {
            center: lam.center,
            radius: lam.radius,
        }
    }
}

impl From<MetalSphere> for Sphere {
    fn from(metal: MetalSphere) -> Self {
        Sphere {
            center: metal.center,
            radius: metal.radius,
        }
    }
}

impl From<DielectricSphere> for Sphere {
    fn from(glass: DielectricSphere) -> Self {
        Sphere {
            center: glass.center,
            radius: glass.radius,
        }
    }
}

#[derive(Clone, Copy)]
pub struct LambertianSphere {
    pub center: Point,
    pub radius: f32,
    pub albedo: Point,
}

impl Hittable for LambertianSphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rng: &mut ThreadRng) -> Option<HitRecord> {
        hit_sphere(self.clone(), r, t_min, t_max, rng)
    }

    fn scatter(&self, _r_in: Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<ScatterResults> {
        let mut scatter_direction = rec.normal + random_unit_vector(rng);
        if scatter_direction.near_zero() { scatter_direction = rec.normal; }
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some(
            ScatterResults {
                ray_dir: scattered,
                norm: rec.normal ,// ??
                attenuation: self.albedo
            }
        )
    }

    fn get_bounding_box(&self) -> Option<AABB> { // Bounding Volume Requirement
        sphere_to_bounding_box(self.clone())
    }
}

impl LambertianSphere {
    pub fn new(center: Point, radius: f32, albedo: Point) -> LambertianSphere {
        LambertianSphere {
            center,
            radius,
            albedo,
        }
    }
}

#[derive(Clone, Copy)]
pub struct MetalSphere {
    pub center: Point,
    pub radius: f32,
    pub albedo: Point,
    fuzz: f32,
}

impl MetalSphere {
    pub fn new(center: Point, radius: f32, albedo: Point, fuzz: f32) -> MetalSphere {
        MetalSphere {
            center,
            radius,
            albedo,
            fuzz,
        }
    }
}

impl Hittable for MetalSphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rng: &mut ThreadRng) -> Option<HitRecord> {
        hit_sphere(self.clone(), r, t_min, t_max, rng)
    }

    fn scatter(&self, r_in: Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<ScatterResults> {
        let reflected = reflect(unit_vector(r_in.direction), rec.normal);
        // Reflection Requirement
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere(rng));
        let attenuation = self.albedo;
        if dot(scattered.direction, rec.normal) > 0.0 {
            return Some(
                ScatterResults{
                    ray_dir: scattered,
                    norm: rec.normal, //?
                    attenuation
                }
            );
        }
       return None;
    }

    fn get_bounding_box(&self) -> Option<AABB> {
        sphere_to_bounding_box(self.clone())
    }
}

#[derive(Clone, Copy)]
pub struct DielectricSphere {
    pub center: Point,
    pub radius: f32,
    ir: f32,
}

impl DielectricSphere {
    pub fn new(center: Point, radius: f32, ir: f32) -> DielectricSphere {
        DielectricSphere {
            center,
            radius,
            ir,
        }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * pow((1.0 - cosine), 5)
    }
}

impl Hittable for DielectricSphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rng: &mut ThreadRng) -> Option<HitRecord> {
        hit_sphere(self.clone(), r, t_min, t_max, rng)
    }

    fn scatter(&self, r_in: Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<ScatterResults> {
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };
        let attenuation = Point::new(1.0, 1.0, 1.0);
        let unit_direction = unit_vector(r_in.direction);
        let dot = dot(-unit_direction, rec.normal);
        let cos_theta = if dot < 1.0 { dot } else { 1.0 };

        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let rand_f32 = rng.gen_range(0.0..1.0);
        let reflectance_bool = DielectricSphere::reflectance(cos_theta, refraction_ratio) > rand_f32;

        let direction = if cannot_refract || reflectance_bool {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };

        let scattered = Ray { origin: rec.p, direction };
        Some(
            ScatterResults{
                ray_dir: scattered,
                norm: direction,
                attenuation
            }
        )
    }

    fn get_bounding_box(&self) -> Option<AABB> {
        sphere_to_bounding_box(self.clone())
    }
}

