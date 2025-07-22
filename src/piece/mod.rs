pub mod movement;

use crate::board::{self, Coord, Offset};
use crate::piece::movement::{Direction, Movement};
use bevy::prelude::*;
use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};
use std::hint::unreachable_unchecked;
use bevy::ecs::define_label;

#[derive(Debug, Clone, Copy, Default)]
pub enum PawnState {
    #[default]
    NotYet,
    Just,
    Already,
}

#[derive(Debug, Clone, Copy)]
pub enum Kind {
    Pawn { state: PawnState },
    Rook { state: bool },
    Knight,
    Bishop,
    Queen,
    King { state: bool },
}
impl Kind {
    // TODO add special move (and its enum) & filter moves
    pub fn move_set(&self, from: Coord, color: PieceColor) -> HashSet<Movement> {
        let build_move = |offset| Movement::new(from, (from + offset).unwrap(), offset.try_into().ok());
        match self {
            Self::Pawn { state } => {
                let sign = color.sign();
                let mut res = vec![
                    Offset::new(0, sign),
                    Offset::new(1, sign),
                    Offset::new(-1, sign),
                ];
                if matches!(state, PawnState::NotYet) {
                    res.push(Offset::new(0, sign * 2));
                }
                res.into_iter().map(build_move).collect()
            }
            Self::Rook { .. } | Self::Bishop => {
                let get_offset = match self {
                    Self::Bishop => |i| Offset::new(i, i),
                    Self::Rook { .. } => |i| Offset::new(i, 0),
                    _ => unsafe { unreachable_unchecked() },
                };
                let mut res = Vec::new();
                for i in 0..board::SIZE as isize {
                    res.extend(get_offset(i).rotations());
                }
                res.into_iter().map(build_move).collect()
            }
            Self::Knight => {
                Offset::new(1, 2).rotations().into_iter().map(build_move).collect()
            }
            Self::Queen => {
                let mut res = HashSet::new();
                res.extend(Self::Bishop.move_set(from, color));
                res.extend(Self::Rook { state: Default::default() }.move_set(from, color));
                res
            }
            Self::King { .. } => {
                let mut res = Vec::new();
                res.extend(Offset::new(1, 0).rotations());
                res.extend(Offset::new(1, 1).rotations());

                res.into_iter().map(build_move).collect()
            }
        }
    }
}
impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Pawn { .. } => write!(f, "pawn"),
            Kind::Rook { .. } => write!(f, "rook"),
            Kind::Knight => write!(f, "knight"),
            Kind::Bishop => write!(f, "bishop"),
            Kind::Queen => write!(f, "queen"),
            Kind::King { .. } => write!(f, "king"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum PieceColor {
    Black,
    #[default]
    White,
}
impl Display for PieceColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PieceColor::White => write!(f, "white"),
            PieceColor::Black => write!(f, "black"),
        }
    }
}
impl PieceColor {
    #[inline]
    pub const fn opposite(self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
    #[inline]
    pub const fn first_row(&self) -> usize {
        match self {
            Self::Black => board::SIZE - 1,
            Self::White => 0,
        }
    }
    #[inline]
    pub const fn sign(&self) -> isize {
        match self {
            Self::Black => -1,
            Self::White => 1,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Piece {
    pub kind: Kind,
    pub color: PieceColor,
}

impl Piece {
    #[inline]
    pub const fn new(kind: Kind, color: PieceColor) -> Self {
        Self { kind, color }
    }
}
impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}.png", self.color, self.kind)
    }
}

#[derive(Bundle)] // add transform
pub struct PieceBundle {
    pub piece: Piece,
    pub sprite: Sprite,
    pub transform: Transform,
    pub valid_pos: Coord,
    pub draggable: Draggable,
}

#[derive(Component, Debug, Clone)]
pub struct Draggable;

#[derive(Resource, Default)]
pub struct DragState {
    pub entity: Option<Entity>,
    pub offset: Vec2,
}
