use std::cmp::Ordering;
use crate::board::{Coord, Offset};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum Direction {
    #[default]
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}
impl TryFrom<Offset> for Direction {
    type Error = ();
    fn try_from(value: Offset) -> Result<Self, Self::Error> {
        match (value.x, value.y) {
            (0, 0) => Err(()),
            (x, 0) if x.is_positive() => Ok(Self::Right),
            (x, 0) => Ok(Self::Left),
            (0, y) if y.is_positive() => Ok(Self::Up),
            (0, y) => Ok(Self::Down),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Movement {
    from: Coord,
    to: Coord,
    direction: Option<Direction>,
}

impl Movement {
    #[inline]
    pub const fn new(from: Coord, to: Coord, direction: Option<Direction>) -> Self {
        Self {
            from,
            to,
            direction,
        }
    }
}
