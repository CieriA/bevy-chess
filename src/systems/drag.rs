use crate::board;
use crate::board::Coord;
use crate::piece::{DragState, Draggable, Piece};
use crate::utils::{cursor_pos, get_position};
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Entity, MouseButton, Query, Res, ResMut, Sprite, Transform, Window, With};

pub fn drag_start(
    mut drag_state: ResMut<DragState>,
    window: Query<&Window>,
    input: Res<ButtonInput<MouseButton>>,
    mut drag_ent: Query<(Entity, &Transform, &Sprite), With<Draggable>>,
) {
    if drag_state.entity.is_some() {
        return;
    }
    if input.just_pressed(MouseButton::Left) {
        let window = window.single().unwrap();
        if let Some(cursor_pos) = cursor_pos(window) {
            for (entity, transform, sprite) in drag_ent.iter_mut() {
                let pos = transform.translation.truncate();
                let size = sprite.custom_size.unwrap_or_default() / 2.;
                let overlapping = cursor_pos - pos;
                if overlapping.x.abs() < size.x && overlapping.y.abs() < size.y {
                    drag_state.entity = Some(entity);
                    drag_state.offset = overlapping;
                    break;
                }
            }
        }
    }
}

pub fn dragging(
    drag_state: Res<DragState>,
    window: Query<&Window>,
    mut drag_ent: Query<&mut Transform, With<Draggable>>,
) {
    if let Some(entity) = drag_state.entity {
        let window = window.single().unwrap();
        if let Some(cursor_pos) = cursor_pos(window)
            && let Ok(mut transform) = drag_ent.get_mut(entity)
        {
            transform.translation.x = cursor_pos.x - drag_state.offset.x;
            transform.translation.y = cursor_pos.y - drag_state.offset.y;
            transform.translation.z = 1.;
        }
    }
}

pub fn drag_end(
    mut drag_state: ResMut<DragState>,
    input: Res<ButtonInput<MouseButton>>,
    mut drag_ent: Query<(&mut Transform, &mut Coord), With<Draggable>>,
    window: Query<&Window>,
) {
    if input.just_released(MouseButton::Left) {
        if let Some(entity) = drag_state.entity
            && let Ok((mut transform, mut coord)) = drag_ent.get_mut(entity)
        {
            let window = window.single().unwrap();
            let cell_size = window.width() / board::SIZE as f32;

            // TODO add logic here to check if the move is valid
            let valid_mov = true; // this will be the result of the function

            if valid_mov
                && let Some(cursor) = cursor_pos(window)
                && let Some(pos) = get_position(&cursor, window)
            {
                *coord = pos;
            }
            transform.translation.x = (coord.x as f32 + 0.5) * cell_size;
            transform.translation.y = (coord.y as f32 + 0.5) * cell_size;
            transform.translation.z = 0.;
        }
        drag_state.entity = None;
        drag_state.offset = Vec2::ZERO;
    }
}
