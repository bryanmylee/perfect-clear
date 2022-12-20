use crate::utils::point::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    pub fn get_offset(&self) -> Point {
        match self {
            Direction::Down => Point::new(0, -1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }
}
