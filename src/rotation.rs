use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Orientation {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}

impl Orientation {
    pub fn rotated(&self, by: &Rotation) -> Orientation {
        match self {
            Self::North => match by {
                Rotation::Clockwise => Self::East,
                Rotation::AntiClockwise => Self::West,
                Rotation::Half => Self::South,
            },
            Self::South => match by {
                Rotation::Clockwise => Self::West,
                Rotation::AntiClockwise => Self::East,
                Rotation::Half => Self::North,
            },
            Self::East => match by {
                Rotation::Clockwise => Self::South,
                Rotation::AntiClockwise => Self::North,
                Rotation::Half => Self::West,
            },
            Self::West => match by {
                Rotation::Clockwise => Self::North,
                Rotation::AntiClockwise => Self::South,
                Rotation::Half => Self::East,
            },
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rotation {
    Clockwise = 0,
    AntiClockwise = 1,
    Half = 2,
}
