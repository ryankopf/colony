use super::prelude::*;

pub fn mouse_click_input(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut event: EventWriter<ObjectFinderEvent>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        // let window = if let RenderTarget::Window(id) = camera.target {
        //     windows.get(id).unwrap()
        // } else {
        //     windows.get_primary().unwrap()
        // };
        let (camera, camera_transform) = q_camera.single();
        let window = windows.get_primary().unwrap();//get_window(WindowId::primary()).unwrap();
        if let Some(screen_pos) = window.cursor_position() {
            // get the size of the window
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    
            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    
            // matrix for undoing the projection and camera transform
            let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    
            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    
            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();

            // get a Position
            let position = Position { x: (world_pos.x / TILE_SIZE) as i32, y: (world_pos.y / TILE_SIZE) as i32, z: 0 };
    
            //eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
            // eprintln!("Position: {}/{}", position.x, position.y);
            event.send(ObjectFinderEvent { position });
        }
    }

    // if mouse_button_input.pressed(MouseButton::Left) {
    //     info!("left mouse currently pressed");
    // }
    // if mouse_button_input.just_released(MouseButton::Left) {
    //     info!("left mouse just released");
    // }
}

pub fn object_finder_system(
    mut commands: Commands,
    mut event: EventReader<ObjectFinderEvent>,
    mut people: Query<(&Position, &mut Brain)>,
    //tile_hash: Res<TileHash>,
) {
    for event in event.iter() {
        for (position, mut brain) in people.iter_mut() {
            if position == &event.position {
                info!("found a person at {}/{}", position.x, position.y);
            }
        }
    }
}

pub struct ObjectFinderEvent {
    pub position: Position
}