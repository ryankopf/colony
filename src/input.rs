//mod super::prelude;
use crate::prelude::*;

pub fn keyboard_input(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera>>
) {
    for mut transform in camera.iter_mut() {
        let move_speed = 16.0;
        //transform.translation.x += 5.0;
        let mut next_position = transform.translation;
        if input.pressed(KeyCode::Up) || input.pressed(KeyCode::W) {
            next_position.y += move_speed;
        } else if input.pressed(KeyCode::Down) || input.pressed(KeyCode::S) {
            next_position.y -= move_speed;
        } else if input.pressed(KeyCode::Left) || input.pressed(KeyCode::A) {
            next_position.x -= move_speed;
        } else if input.pressed(KeyCode::Right) || input.pressed(KeyCode::D) {
            next_position.x += move_speed;
        }
        transform.translation = next_position;
        if (next_position.x >= -15.0) && (next_position.x < VIEWAREA_WIDTH as f32 * MAP_WIDTH as f32) {
            if (next_position.y >= -15.0) && (next_position.y < VIEWAREA_HEIGHT as f32 * MAP_LENGTH as f32) {
                //transform.translation = next_position;
            }
        }
        //transform.translation = next_position;
    }
}

pub fn scrollwheel_input(
    mut commands: Commands,
    mut scroll_evr: EventReader<MouseWheel>,
    mut camera: Query<&mut Transform, With<Camera>>
) {
    for mut transform in camera.iter_mut() {
        let move_speed = 16.0;
        //transform.translation.x += 5.0;
        let mut next_position = transform.translation;
        for ev in scroll_evr.iter() {
            if ev.y > 0.0 {
                next_position.z += move_speed;
            } else if ev.y < 0.0 {
                next_position.z -= move_speed;
            }
        }
        transform.translation = next_position;
    }
}