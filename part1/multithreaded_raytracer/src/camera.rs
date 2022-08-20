use rand::rngs::ThreadRng;

use crate::Point;
use crate::ray::Ray;
use crate::utility::{cross, random_in_unit_disk, unit_vector};

pub struct Camera {
    pub lookfrom: Point,
    pub lookat: Point,
    pub vup: Point,

    pub vfov: f32,
    pub aspect_ratio: f32,
    pub aperture: f32,
    pub focus_dist: f32,

    origin: Point,
    lower_left_corner: Point,
    horizontal: Point,
    vertical: Point,
    u: Point,
    v: Point,

    lens_radius: f32,
}

pub trait Cast: Sync + Send {
    fn get_ray(&self, s: f32, t: f32, rng: &mut ThreadRng) -> Ray;
}

impl Cast for Camera {
    fn get_ray(&self, s: f32, t: f32, rng: &mut ThreadRng) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset,
                 self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}

impl Camera {
    pub fn new(lookfrom: Point,
               lookat: Point,
               vup: Point,
               vfov: f32,
               aspect_ratio: f32,
               aperture: f32,
               focus_dist: f32)
               -> Camera {
        // Camera requirement.
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let _focal_length = 1.0;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;


        Camera {
            lookfrom,
            lookat,
            vup,
            vfov,
            aspect_ratio,
            aperture,
            focus_dist,
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }
}