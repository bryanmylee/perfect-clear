use crate::point::Point;

#[derive(Debug, Clone)]
pub enum PieceType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub kind: PieceType,
    pub center: Point<isize>,
    pub orientation: Orientation,
}

#[derive(Debug, Clone)]
pub enum Orientation {
    North,
    South,
    East,
    West,
}
