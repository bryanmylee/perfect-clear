use crate::point::Point;

#[derive(Debug)]
pub enum PieceType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Debug)]
pub struct Piece {
    pub kind: PieceType,
    pub center: Point<isize>,
    pub orientation: Orientation,
}

#[derive(Debug)]
pub enum Orientation {
    North,
    South,
    East,
    West,
}
