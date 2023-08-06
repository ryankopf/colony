//mod super::prelude;
use crate::prelude::*;

pub fn keyboard_input(
    _commands: Commands,
    input: Res<Input<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera>>,
    mut gamestate: ResMut<State<GameState>>,
    mut nextstate: ResMut<bevy::ecs::schedule::NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Space) {
        // Pause or Unpause.
        match gamestate.get() {
            GameState::InGame => {
                nextstate.set(GameState::Paused);
            }
            GameState::Paused => {
                nextstate.set(GameState::InGame);
            }
            _ => {}
        }
    }
    for mut transform in camera.iter_mut() {
        let move_speed = 16.0;
        //transform.translation.x += 5.0;
        let mut next_position = transform.translation;
        if input.any_pressed([KeyCode::Up, KeyCode::W]) {//pressed(KeyCode::Up) || input.pressed(KeyCode::W) {
            next_position.y += move_speed;
        } else if input.any_pressed([KeyCode::Down, KeyCode::S]) {
            next_position.y -= move_speed;
        } else if input.any_pressed([KeyCode::Left, KeyCode::A]) {
            next_position.x -= move_speed;
        } else if input.any_pressed([KeyCode::Right, KeyCode::D]) {
            next_position.x += move_speed;
        }
        transform.translation = next_position;
        if (next_position.x >= -15.0) && (next_position.x < VIEWAREA_WIDTH as f32 * MAP_WIDTH as f32) && (next_position.y >= -15.0) && (next_position.y < VIEWAREA_HEIGHT as f32 * MAP_LENGTH as f32) {
            //transform.translation = next_position;
        }
        //transform.translation = next_position;
    }
}

pub fn scrollwheel_input(
    _commands: Commands,
    mut scroll_evr: EventReader<MouseWheel>,
    //mut camera: Query<&mut Transform, With<Camera>>
    mut camera: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for mut projection in camera.iter_mut() {
        let mut next_zoom = projection.scale;
        for ev in scroll_evr.iter() {
            if ev.y > 0.0 {
                next_zoom += 0.1;
            } else if ev.y < 0.0 {
                next_zoom -= 0.1;
            }
        }
        // Limit next_zoom to between 0.5 and 1.5
        next_zoom = next_zoom.min(1.5).max(0.5);
        projection.scale = next_zoom;
    }
    // for mut transform in camera.iter_mut() {
    //     let move_speed = 16.0;
    //     //transform.translation.x += 5.0;
    //     let mut next_position = transform.translation;
    //     for ev in scroll_evr.iter() {
    //         if ev.y > 0.0 {
    //             next_position.z += move_speed;
    //         } else if ev.y < 0.0 {
    //             next_position.z -= move_speed;
    //         }
    //     }
    //     transform.translation = next_position;
    // }
}