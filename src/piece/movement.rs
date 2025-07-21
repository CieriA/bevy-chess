use crate::board::Coord;

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone)]
pub struct Movement {
    from: Coord,
    to: Coord,
    direction: Direction,
}

impl Movement {
    #[inline]
    pub const fn new(from: Coord, to: Coord, direction: Direction) -> Self {
        Self {
            from,
            to,
            direction,
        }
    }
}
