#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: i16,
    pub y: i16,
}
impl Vector {
    pub fn new(x: i16, y: i16) -> Self {
        Vector { x, y }
    }
}

use std::convert::TryFrom;
pub fn add(p: &Point, v: &Vector, max_x: u16, max_y: u16) -> Point {
    let mut px = i32::from(p.x);
    let mut py = i32::from(p.y);
    px = px + i32::from(v.x);
    py = py + i32::from(v.y);
    if px < 0 {
        px = px + i32::from(max_x);
    } else if px >= i32::from(max_x) {
        px = px - i32::from(max_x);
    }
    if py < 0 {
        py = py + i32::from(max_y);
    } else if py >= i32::from(max_y) {
        py = py - i32::from(max_y);
    }
    let pxx = u16::try_from(px).unwrap();
    let pyy = u16::try_from(py).unwrap();
    Point::new(pxx, pyy)
}
