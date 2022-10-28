use crate::point::Point;
use crate::rotation::Rotation;

#[derive(Debug)]
pub enum PieceKind {
    I,
    O,
    S,
    Z,
    T,
    L,
    J,
}

#[derive(Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub center: Point<isize>,
    pub rotation: Rotation,
}
