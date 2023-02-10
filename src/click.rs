use super::prelude::*;

// Make plugin.
pub struct ClickPlugin;

impl Plugin for ClickPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ObjectFinderEvent>()
            .add_system(mouse_click_input)
            .add_system(mouse_drag_system)
            .add_system(object_finder_system)
            .insert_resource(Dragging { dragging: false, start_position: None })
        ;
    }
}

pub fn mouse_click_input(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut event: EventWriter<ObjectFinderEvent>,
    mut dragging: ResMut<Dragging>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
        let window = windows.get_primary().unwrap();
        let mut position = None;
        if let Some(screen_pos) = window.cursor_position() {
            position = Some(mouse_to_position(camera, camera_transform, window, screen_pos));
        }
        if let Some(position) = position {
            event.send(ObjectFinderEvent { position });
            dragging.dragging = true;
            dragging.start_position = Some(position);
        }
    }
    if mouse_button_input.just_released(MouseButton::Left) {
        // Maybe we send an event here? Or we iterate here through all "Highlighted" and mark them "Selected"?
        dragging.dragging = false;
    }
}

pub fn mouse_drag_system(
    mut commands: Commands,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    dragging: Res<Dragging>,
    positions: Query<(Entity, &Position, Option<&Highlighted>)>,
    // highlights: Query<(Entity, &Position), With<Highlighted>>,
    highlightboxes: Query<(Entity, &Parent), With<HighlightBox>>
) {
    // Yes, this runs all the time.
    // But it's not a problem, because it's a no-op if we're not dragging.
    if !dragging.dragging { return; }
    if dragging.start_position == None { return; }
    let start_position = dragging.start_position.unwrap();
    let (camera, camera_transform) = q_camera.single();
    let window = windows.get_primary().unwrap();
    let mut end_position = None;
    if let Some(screen_pos) = window.cursor_position() {
        end_position = Some(mouse_to_position(camera, camera_transform, window, screen_pos));
    }
    if end_position == None { return; }
    let end_position = end_position.unwrap();
    println!("Dragging from {}/{} to {}/{}", start_position.x, start_position.y, end_position.x, end_position.y);
    // Now just take all objects with a position that matches and mark them as "Highlighted".
    // Somehow only allow the types I want to be highlighted. Foragable. Unit. Choppable. Food. Storable.
    for (entity, pos, highlighted) in positions.iter() {
        if (start_position.x.min(end_position.x) <= pos.x) && (pos.x <= start_position.x.max(end_position.x) && (start_position.y.min(end_position.y) <= pos.y) && (pos.y <= start_position.y.max(end_position.y))) {
            if (highlighted.is_some()) { continue; }
            // println!("Entity {} is highlighted", entity);
            let highlight_box = commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 0.2),
                    custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, pos.z as f32 + 0.1),//pos.x as f32, pos.y as f32, pos.z as f32 + 0.1),
                ..default()
            }).insert(HighlightBox).id();
            commands.entity(entity).insert(Highlighted { ..default() });
            commands.entity(entity).add_child(highlight_box);
        } else {
            if (highlighted.is_none()) { continue; }
            // println!("Entity {} is not highlighted", entity);
            commands.entity(entity).remove::<Highlighted>();
            for (highlight_box, parent) in highlightboxes.iter() {
                if (parent.get() == entity) {
                    commands.entity(highlight_box).despawn();
                }
            }
        }
    }
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

fn mouse_to_position(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    window: &Window,
    screen_pos: Vec2,
) -> Position {
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

    // eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
    // eprintln!("Position: {}/{}", position.x, position.y);
    // event.send(ObjectFinderEvent { position });
    // dragging.dragging = true;
    // dragging.start_position = Some(position);
    return position
}