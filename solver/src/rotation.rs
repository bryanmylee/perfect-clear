#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Orientation {
    North,
    South,
    East,
    West,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rotation {
    Clockwise,
    AntiClockwise,
    Half,
}
