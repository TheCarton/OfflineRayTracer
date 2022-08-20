// The following is based heavily on Peter Shirley's  <ptrshrl@gmail.com>
// Ray Tracing in One Weekend
// https://raytracing.github.io/books/RayTracingInOneWeekend.html

use std::thread;

use progress_bar::*;
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::camera::{Camera, Cast};
use crate::hittable_list::{CheckHits, HittableList};
use crate::point::Point;
use crate::ppm::PPM;
use crate::ray::Ray;
use crate::row_data::RowData;
use crate::sphere::{DielectricSphere, LambertianSphere, MetalSphere};
use crate::utility::{dot, random_unit_vector, unit_vector};

mod hittable;
mod hittable_list;
mod camera;
mod utility;
mod point;
mod ray;
mod ppm;
mod performance_stats;
mod material;
mod sphere;
mod aabb;
mod bvh;
mod row_data;
mod scatter_results;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1600;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
const IMAGE_SIZE: usize = (IMAGE_HEIGHT * IMAGE_WIDTH * 3) as usize;

fn get_canvas_color(r: Ray) -> Point {

    let unit_direction = unit_vector(r.direction);
    let t = 1.6 * (unit_direction.y + 1.0);
    return (1.0 - t) * Point::new(0.9, 0.9, 0.9) + t * Point::new(0.5, 0.7, 1.0);
}

fn row_color(row_j: u32, w: &HittableList, c: &Camera, depth: i32, samples_per_pixel: i32) -> RowData {
    let image_rgb_i = row_j * IMAGE_WIDTH * 3;
    let mut rng = rand::thread_rng();
    let mut row_data = RowData::new(image_rgb_i);

    for pixel_i in 0..IMAGE_WIDTH {
        let mut color = Point::new(0.0, 0.0, 0.0);
        for _ in 0..samples_per_pixel {
            let u = (pixel_i as f32 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f32;
            let v = (row_j as f32 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT - 1) as f32;
            let r = c.get_ray(u, v, &mut rng);
            let sample = ray_color(r, w, depth, &mut rng);
            color = color + sample;
        }
        row_data.push_color(color, samples_per_pixel);
        inc_progress_bar();
    }
    row_data
}

fn ray_color(r: Ray, world: &HittableList, depth: i32, rng: &mut ThreadRng) -> Point {
    if depth <= 0 { return Point::new(0.0, 0.0, 0.0); }
    if let Some(rec) = world.get_hits(r, 0.001, f32::INFINITY, rng) {
        if let Some(scatter_results) = rec.scatter_results {
            let scattered_ray = scatter_results.ray_dir;
            let attenuation = scatter_results.attenuation;
            return attenuation * ray_color(scattered_ray, world, depth - 1, rng);
        } else {
            return Point::new(0.0, 0.0, 0.0);
        }
    }
    return get_canvas_color(r);
}

fn main() -> std::io::Result<()> {


    // image

    let samples_per_pixel = 300;
    let depth = 500;

    // progress bar
    init_progress_bar(IMAGE_SIZE / 3 as usize);
    set_progress_bar_action("Loading", Color::Blue, Style::Bold);

    // world

    let mut world = HittableList::new();


    let a = Point::new(0.0, 1.0, 0.0);
    let pink = Point::new(1.0, 0.8, 0.801);
    let sphere1 = MetalSphere::new(a, 1.0, pink, 0.0);
    world.add(sphere1);

    let b = Point::new(-2.0, 1.0, 0.0);
    let sphere2 = DielectricSphere::new(b, 1.0, 1.5);
    world.add(sphere2);

    let c = Point::new(2.0, 1.0, 0.0);
    let red = Point::new(1.0, 0.0, 0.0);
    let sphere3 = LambertianSphere::new(c, 1.0, red);
    world.add(sphere3);
    let w = &world;

    // CAMERA
    let lookfrom = Point::new(0.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Point::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(lookfrom, lookat, vup, 90.0, ASPECT_RATIO, aperture, dist_to_focus);
    let c = &camera;

    let filename = "output.ppm".to_owned();
    let mut image_rgb: [u8; IMAGE_SIZE] = [0; IMAGE_SIZE];
    thread::scope(|s| {
        let mut handles = vec![];
        for row_j in 0..IMAGE_HEIGHT {
            let handle = s.spawn(move || {
                row_color(row_j, w, c, depth, samples_per_pixel)
            });
            handles.push(handle);
        }
        for handle in handles {
            let row_data = handle.join().unwrap();
            let mut i = row_data.index as usize;
            for val in row_data.rbg_values {
                image_rgb[i] = val;
                i += 1;
            }
        }
    });


    let image = PPM {
        height: IMAGE_HEIGHT,
        width: IMAGE_WIDTH,
        data: Vec::from(image_rgb),
    };
    image.write_file(&filename).expect("Failed to write to PPM.");
    finalize_progress_bar();


    Ok(())
}

