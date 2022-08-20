use rand::Rng;
use rand::rngs::ThreadRng;

use crate::Point;

pub fn clamp(input: f32, min: f32, max: f32) -> f32 {
    if input < min {
        return min;
    } else if input > max {
        return max;
    }
    return input;
}

pub fn refract(uv: Point, n: Point, etai_over_etat: f32) -> Point {
    let negative_uv = -uv;
    let dot = dot(negative_uv, n);
    let cos_theta = if dot < 1.0 { dot } else { 1.0 };
    let r_out_perp = etai_over_etat * (uv + (cos_theta * n));
    let a = -((1.0 - r_out_perp.length_squared()).sqrt());
    let r_out_parallel = a * n;
    r_out_perp + r_out_parallel
}

pub fn random_point_range(rng: &mut ThreadRng, min: f32, max: f32) -> Point {
    Point::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
}

pub fn reflect(v: Point, n: Point) -> Point {
    let s = 2.0 * dot(v, n);
    v.clone() - s * n.clone()
}

pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Point {
    loop {
        let p = random_point_range(rng, -1.0, 1.0);
        if p.length_squared() >= 1.0 { continue; }
        return p;
    }
}

pub fn random_unit_vector(rng: &mut ThreadRng) -> Point {
    unit_vector(random_in_unit_sphere(rng))
}

pub fn cross(u: Point, v: Point) -> Point {
    let x = u[1] * v[2] - u[2] * v[1];
    let y = u[2] * v[0] - u[0] * v[2];
    let z = u[0] * v[1] - u[1] * v[0];
    Point::new(
        x,
        y,
        z,
    )
}

pub fn unit_vector(v: Point) -> Point {
    v / v.length()
}

pub fn dot(u: Point, v: Point) -> f32 {
    u[0] * v[0] +
        u[1] * v[1] +
        u[2] * v[2]
}

pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Point {
    loop {
        let x = rng.gen_range(-1.0..1.0);
        let y = rng.gen_range(-1.0..1.0);
        let p = Point::new(x, y, 0.0);
        if p.length_squared() >= 1.0 { continue; }
        return p;
    }
}

#[cfg(test)]
#[test]
fn dot_prod() {
    let p = Point::new(10.0, 20.0, 30.0);
    let p2 = Point::new(10.0, 10.0, 10.0);
    let prod = dot(p, p2);
    assert_eq!(prod, 600.0);
}

#[test]
fn dot_prod_one_zero() {
    let p = Point::new(0.0, 0.0, 0.0);
    let p2 = Point::new(10.0, 10.0, 10.0);
    let prod = dot(p, p2);
    assert_eq!(prod, 0.0);
}

#[test]
fn dot_prod_negative() {
    let p = Point::new(-1.0, -1.0, -1.0);
    let p2 = Point::new(1.0, 2.0, 3.0);
    let prod = dot(p, p2);
    assert_eq!(prod, -6.0);
}