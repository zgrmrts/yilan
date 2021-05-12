use std::ops::Add;
#[derive(Debug, Copy, Clone, PartialEq)]
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

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, other: Vector) -> Self {
        Self {
            x: (self.x as i32 + other.x as i32) as u16,
            y: (self.y as i32 + other.y as i32) as u16,
        }
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
    let pxx = u16::try_from(px).expect("impossible");
    let pyy = u16::try_from(py).expect("impossible");
    Point::new(pxx, pyy)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_vector_addition() {
        let p = Point::new(15, 16);
        let v = Vector::new(-1, 1);
        let q = p + v;
        assert_eq!(q, Point::new(14, 17));
    }
}
