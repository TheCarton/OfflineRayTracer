use crate::{IMAGE_WIDTH, Point};
use crate::utility::clamp;

pub struct RowData {
    pub index: u32,
    pub rbg_values: [u8; (IMAGE_WIDTH * 3) as usize],
    i: usize,
}

impl RowData {
    pub fn new(index: u32) -> RowData {
        RowData {
            index,
            rbg_values: [255; (IMAGE_WIDTH * 3) as usize],
            i: 0,
        }
    }

    pub fn push_color(&mut self, rgb_point: Point, samples_per_pixel: i32) {
        let scale = 1.0 / samples_per_pixel as f32;
        let r = (scale * rgb_point[0]).sqrt(); // Gamma correction
        let g = (scale * rgb_point[1]).sqrt();
        let b = (scale * rgb_point[2]).sqrt();

        let new_r = (255.0 * clamp(r, 0.0, 1.0)) as u8;
        let new_g = (255.0 * clamp(g, 0.0, 1.0)) as u8;
        let new_b = (255.0 * clamp(b, 0.0, 1.0)) as u8;

        self.rbg_values[self.i] = new_r;
        self.rbg_values[self.i + 1] = new_g;
        self.rbg_values[self.i + 2] = new_b;
        self.i += 3;
    }
}

#[cfg(test)]
#[test]
fn iter_self() {
    let mut cd = RowData::new(66);
    let red = Point::new(1.0, 0.0, 0.0);
    let blue = Point::new(0.0, 1.0, 0.0);
    let green = Point::new(0.0, 0.0, 1.0);

    cd.push_color(red, 1);
    assert_eq!(cd.rbg_values[0], 255);
    assert_eq!(cd.rbg_values[1], 0);
    assert_eq!(cd.rbg_values[2], 0);
    assert_eq!(cd.rbg_values[3], 0);

    cd.push_color(blue, 1);
    assert_eq!(cd.rbg_values[0], 255);
    assert_eq!(cd.rbg_values[1], 0);
    assert_eq!(cd.rbg_values[2], 0);

    assert_eq!(cd.rbg_values[3], 0);
    assert_eq!(cd.rbg_values[4], 255);
    assert_eq!(cd.rbg_values[5], 0);

    assert_eq!(cd.rbg_values[6], 0);

    cd.push_color(green, 1);

    assert_eq!(cd.rbg_values[0], 255);
    assert_eq!(cd.rbg_values[1], 0);
    assert_eq!(cd.rbg_values[2], 0);

    assert_eq!(cd.rbg_values[3], 0);
    assert_eq!(cd.rbg_values[4], 255);
    assert_eq!(cd.rbg_values[5], 0);

    assert_eq!(cd.rbg_values[6], 0);
    assert_eq!(cd.rbg_values[7], 0);
    assert_eq!(cd.rbg_values[8], 255);
}