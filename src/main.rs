use bevy::prelude::*;
use bevy::time::FixedTimestep;
mod startup;
use startup::*;
mod map;
use map::*;
mod components;
use components::*;
mod resources;
mod constants;
mod moverandom_system;
mod input;
mod prelude;
mod monstergenerator_system;
use monstergenerator_system::*;
mod movetoward_system;
use movetoward_system::*;
mod seasons;
mod needs;
mod text_system;
mod names_system;
mod statusdisplay_system;
mod namegiving_system;
mod thinking_system;
mod task_system;
mod window_system;
mod click;
mod spoilage_system;
mod mods;
use mods::*;

fn main() {
    //println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_sprites)
        .add_startup_system(generate_map)
        .add_startup_system(startup)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_text)
        .add_startup_system(set_window_icon)
        .add_startup_system(set_window_maximized)
        .add_startup_system(text_test)
        .add_state(GameState::Paused)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(movement_random),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(monster_generator),
        )
        .add_plugin(MovementPlugin)
        .add_plugin(SeasonsPlugin)
        .add_plugin(NeedsPlugin)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(status_display_system),
        )
        .add_plugin(ThinkingPlugin)
        .add_plugin(TaskPlugin)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(spoilage_system),
        )
        .add_system(remove_bad_positions)
        .add_system(namegiving_system)
        .add_system(names_system)
        .add_system(text_update_system)
        .add_system(movement_toward_attackable)
        .add_event::<FoodNotifEvent>()
        .add_event::<ObjectFinderEvent>()
        .add_system(object_finder_system)
        .add_system(keyboard_input)
        .add_system(scrollwheel_input)
        .add_system(mouse_click_input)
        .add_system(bevy::window::close_on_esc)
        .run();
}


fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation.x = TILE_SIZE * 19.0;
    camera.transform.translation.y = TILE_SIZE * 11.0;
    commands.spawn(camera);
}

fn remove_bad_positions(
    mut commands: Commands,
    query: Query<(Entity, &Position), Without<MapTile>>,
    tiles: Query<(&Position, &TileType), With<MapTile>>,
) {
    let mut tiletypes: std::collections::HashMap<Position, TileType> = std::collections::HashMap::new();
    for (tile_position, tile_type) in tiles.iter() {
        tiletypes.insert(tile_position.clone(), tile_type.clone());
    }
    for (entity, position) in query.iter() {
        if tiletypes.contains_key(&position) {
            if tiletypes.get(&position).unwrap() == &TileType::Wall {
                commands.entity(entity).despawn();
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
}