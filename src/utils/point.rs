use std::ops::{Add, AddAssign, Sub};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

// Structs with generics are not supported by `wasm_bindgen`, therefore use a concrete `Point` type.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ISizePoint {
    pub x: isize,
    pub y: isize,
}

impl ISizePoint {
    pub fn new(x: isize, y: isize) -> ISizePoint {
        ISizePoint { x, y }
    }
}

impl Add for ISizePoint {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        ISizePoint::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for ISizePoint {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for ISizePoint {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        ISizePoint::new(self.x - other.x, self.y - other.y)
    }
}
