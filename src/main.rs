use crate::events::*;
use crate::systems::{drag::*, *};
use bevy::prelude::*;

mod board;
mod events;
mod piece;
mod systems;
mod utils;

const WINDOW_SIZE: f32 = board::CELL_SIZE * board::SIZE as f32;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess".to_string(),
                resolution: (WINDOW_SIZE, WINDOW_SIZE).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_event::<Promote>()
        .add_event::<Check>()
        .add_event::<GameOver>()
        .add_systems(Startup, (setup, place_board).chain())
        .add_systems(Update, (drag_start, dragging, drag_end).chain())
        .run();
}
