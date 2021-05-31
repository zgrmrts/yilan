use super::point::Vector;
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn delta(&self) -> Vector {
        match self {
            Direction::Up => Vector::new(0, -1),
            Direction::Down => Vector::new(0, 1),
            Direction::Left => Vector::new(-1, 0),
            Direction::Right => Vector::new(1, 0),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_direction() {
        assert_eq!(Direction::Right.delta(), Vector::new(1, 0));
    }
}
