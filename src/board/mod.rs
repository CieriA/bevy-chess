pub mod points;
#[cfg(test)]
mod tests;
pub use points::{Coord, Offset};

use crate::piece::{Kind, Piece, PieceColor};
use bevy::prelude::{Component, Resource};
use std::collections::HashMap;
use std::ops::Add;

pub const SIZE: usize = 8;
pub const CELL_SIZE: f32 = 120.;

#[derive(Resource, Debug, Clone)]
pub struct BoardState(pub HashMap<Coord, Piece>);

fn placement(x: usize) -> Option<Kind> {
    match x {
        0 | 7 => Some(Kind::Rook {
            state: Default::default(),
        }),
        1 | 6 => Some(Kind::Knight),
        2 | 5 => Some(Kind::Bishop),
        3 => Some(Kind::Queen),
        4 => Some(Kind::King {
            state: Default::default(),
        }),
        _ => None,
    }
}

impl Default for BoardState {
    fn default() -> Self {
        let mut pieces = HashMap::new();
        for x in 0..SIZE {
            let color = PieceColor::White;
            let pos = Coord::new(x, color.first_row()).unwrap();
            pieces.insert(pos, Piece::new(placement(pos.x).unwrap(), color));

            let pos = Coord::new(x, color.first_row() + 1).unwrap();
            pieces.insert(
                pos,
                Piece::new(
                    Kind::Pawn {
                        state: Default::default(),
                    },
                    color,
                ),
            );

            let color = PieceColor::Black;
            let pos = Coord::new(x, color.first_row()).unwrap();
            pieces.insert(pos, Piece::new(placement(pos.x).unwrap(), color));

            let pos = Coord::new(x, color.first_row() - 1).unwrap();
            pieces.insert(
                pos,
                Piece::new(
                    Kind::Pawn {
                        state: Default::default(),
                    },
                    color,
                ),
            );
        }

        Self(pieces)
    }
}
