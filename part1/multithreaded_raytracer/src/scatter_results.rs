use crate::{Point, Ray};

#[derive(Clone, Copy)]
pub struct ScatterResults {
    pub(crate) ray_dir: Ray,
    pub(crate) norm: Point,
    pub(crate) attenuation: Point,
}