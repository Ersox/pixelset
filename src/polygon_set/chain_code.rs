/// A direction in the 8-connected Moore neighborhood, encoded 0..=7.
/// Directions are ordered clockwise starting from East.
///
/// ```text
///     6   7   0
///     (N) (NE)(E)
///     5   P   1
///     (NW)    (SE)
///     4   3   2
///     (W) (SW)(S)
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ChainDir {
    East = 0,
    SouthEast = 1,
    South = 2,
    SouthWest = 3,
    West = 4,
    NorthWest = 5,
    North = 6,
    NorthEast = 7,
}

impl ChainDir {
    /// Returns the (dx, dy) offset for this direction.
    /// Note: positive y is downward (row index increases).
    pub fn offset(self) -> (i32, i32) {
        match self {
            Self::East => (1, 0),
            Self::SouthEast => (1, 1),
            Self::South => (0, 1),
            Self::SouthWest => (-1, 1),
            Self::West => (-1, 0),
            Self::NorthWest => (-1, -1),
            Self::North => (0, -1),
            Self::NorthEast => (1, -1),
        }
    }

    /// Rotates this direction one step clockwise (45 degrees).
    pub fn clockwise(self) -> Self {
        match self {
            Self::East => Self::SouthEast,
            Self::SouthEast => Self::South,
            Self::South => Self::SouthWest,
            Self::SouthWest => Self::West,
            Self::West => Self::NorthWest,
            Self::NorthWest => Self::North,
            Self::North => Self::NorthEast,
            Self::NorthEast => Self::East,
        }
    }

    /// Rotates this direction one step counter-clockwise (45 degrees).
    pub fn counter_clockwise(self) -> Self {
        match self {
            Self::East => Self::NorthEast,
            Self::NorthEast => Self::North,
            Self::North => Self::NorthWest,
            Self::NorthWest => Self::West,
            Self::West => Self::SouthWest,
            Self::SouthWest => Self::South,
            Self::South => Self::SouthEast,
            Self::SouthEast => Self::East,
        }
    }

    /// Returns the opposite direction (rotated 180 degrees).
    pub fn back(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::SouthEast => Self::NorthWest,
            Self::South => Self::North,
            Self::SouthWest => Self::NorthEast,
            Self::West => Self::East,
            Self::NorthWest => Self::SouthEast,
            Self::North => Self::South,
            Self::NorthEast => Self::SouthWest,
        }
    }

    /// Converts a u8 to a ChainDir. Panics if out of range.
    pub(crate) fn from_u8(val: u8) -> Self {
        match val {
            0 => Self::East,
            1 => Self::SouthEast,
            2 => Self::South,
            3 => Self::SouthWest,
            4 => Self::West,
            5 => Self::NorthWest,
            6 => Self::North,
            7 => Self::NorthEast,
            _ => panic!("invalid chain direction: {}", val),
        }
    }
}
