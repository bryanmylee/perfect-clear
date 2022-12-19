use crate::point::ISizePoint;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    pub fn get_offset(&self) -> ISizePoint {
        match self {
            Direction::Down => ISizePoint::new(0, -1),
            Direction::Left => ISizePoint::new(-1, 0),
            Direction::Right => ISizePoint::new(1, 0),
        }
    }
}
