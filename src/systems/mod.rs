pub mod drag;

use crate::board::{self, BoardState};
use crate::piece::{DragState, Draggable, PieceBundle};
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window: Query<&Window>) {
    let window = window.single().unwrap();
    let width = window.width();
    let height = window.height();
    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
    ));
    commands.insert_resource(BoardState::default());
    commands.insert_resource(DragState::default());

    let background = asset_server.load("background.png");
    commands.spawn((
        Sprite {
            image: background,
            custom_size: Some(Vec2::new(width, height)),
            anchor: Anchor::BottomLeft,
            ..default()
        },
        Transform::from_translation(Vec3::new(0., 0., -10.)),
    ));
}

pub fn place_board(
    mut commands: Commands,
    board: Res<BoardState>,
    asset_server: Res<AssetServer>,
    window: Query<&Window>,
) {
    let window = window.single().unwrap();
    let cell_size = window.width() / board::SIZE as f32;
    for (coord, piece) in &board.0 {
        commands.spawn(PieceBundle {
            piece: piece.clone(),
            sprite: Sprite {
                image: asset_server.load(piece.to_string()),
                custom_size: Some(Vec2::new(cell_size, cell_size)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                (coord.x as f32 + 0.5) * cell_size,
                (coord.y as f32 + 0.5) * cell_size,
                1.,
            ))
            .with_scale(Vec3::new(0.85, 0.85, 1.)),
            valid_pos: *coord,
            draggable: Draggable,
        });
    }
}
