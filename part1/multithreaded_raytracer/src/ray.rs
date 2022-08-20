use crate::Point;

#[derive(Clone, Copy)]
pub struct Ray {
    pub(crate) origin: Point,
    pub(crate) direction: Point,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Ray {
            origin,
            direction,
        }
    }

    pub fn at(self, t: f32) -> Point {
        self.origin + t * self.direction
    }
}
