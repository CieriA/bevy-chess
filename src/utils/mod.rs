use crate::board;
use crate::board::Coord;
use bevy::math::Vec2;
use bevy::prelude::{Transform, Window};

pub fn cursor_pos(window: &Window) -> Option<Vec2> {
    window
        .cursor_position()
        .map(|pos| Vec2::new(pos.x, window.width() - pos.y))
}

pub fn get_position(pos: &Vec2, window: &Window) -> Option<Coord> {
    let cell_size = window.width() / board::SIZE as f32;
    let x = pos.x / cell_size;
    let y = pos.y / cell_size;
    debug_assert_eq!(x, x.abs());
    debug_assert_eq!(y, y.abs()); // both should be an integer if the code is right
    Coord::new(x as usize, y as usize)
}
