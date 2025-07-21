pub mod movement;

use crate::board;
use crate::board::Coord;
use crate::piece::movement::{Direction, Movement};
use bevy::prelude::*;
use std::fmt::{self, Display, Formatter};

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
