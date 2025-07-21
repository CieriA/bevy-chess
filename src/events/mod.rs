use crate::board::Coord;
use crate::piece::PieceColor;
use crate::piece::movement::Movement;
use bevy::prelude::Event;

#[derive(Event, Debug)]
pub struct GameOver(Option<PieceColor>);

#[derive(Event, Debug)]
pub struct Check(Movement);

#[derive(Event, Debug)]
pub struct Promote(Movement);
