use crate::board::SIZE;
use bevy::prelude::Component;
use std::collections::HashSet;
use std::ops::{Add, Mul};

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub type Coord = Point<usize>;
pub type Offset = Point<isize>;

impl Add<Offset> for Coord {
    type Output = Option<Self>;
    fn add(self, rhs: Offset) -> Self::Output {
        Offset::new(self.x as isize + rhs.x, self.y as isize + rhs.y)
            .try_into()
            .ok()
    }
}
impl Mul<isize> for Offset {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Coord {
    #[inline]
    pub fn new(x: usize, y: usize) -> Option<Self> {
        (x < SIZE && y < SIZE).then_some(Self { x, y })
    }
}
impl Offset {
    #[inline]
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn rotations(self) -> HashSet<Self> {
        let mut res = HashSet::new();
        let permutations = [self, Offset::new(self.y, self.x)];
        for permutation in permutations {
            for x_sign in [-1, 1] {
                for y_sign in [-1, 1] {
                    res.insert(Offset::new(permutation.x * x_sign, permutation.y * y_sign));
                }
            }
        }
        res
    }
}
impl TryFrom<(usize, usize)> for Coord {
    type Error = ();
    #[inline]
    fn try_from((x, y): (usize, usize)) -> Result<Self, Self::Error> {
        Self::new(x, y).ok_or(())
    }
}
impl From<(isize, isize)> for Offset {
    #[inline]
    fn from((x, y): (isize, isize)) -> Self {
        Self::new(x, y)
    }
}
impl TryFrom<Offset> for Coord {
    type Error = ();
    #[inline]
    fn try_from(Offset { x, y }: Offset) -> Result<Self, Self::Error> {
        if x >= 0 && y >= 0 {
            Self::new(x as usize, y as usize).ok_or(())
        } else {
            Err(())
        }
    }
}
