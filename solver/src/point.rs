#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

use std::ops::Add;

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

use std::ops::AddAssign;

impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
