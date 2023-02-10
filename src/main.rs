use bevy::time::FixedTimestep;
mod prelude;
pub use crate::prelude::*;

use retrieve::mod_use;
#[mod_use(click, components, constants, game_ui, input, map, monstergenerator_system, moverandom_system, movetoward_system,
    namegiving_system, names_system, needs, resources, seasons, spoilage_system, startup, statusdisplay_system, task_system, text_system, thinking_system, window_system)]

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
        .add_plugin(MonsterGeneratorPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(SeasonsPlugin)
        .add_plugin(NeedsPlugin)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(status_display_system),
        )
        .add_plugin(GameUiPlugin)
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
        .add_system(keyboard_input)
        .add_system(scrollwheel_input)
        .add_plugin(ClickPlugin)
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
    tiletypes: Res<TileHash>,
) {
    for (entity, position) in query.iter() {
        if tiletypes.hash.contains_key(&position) {
            if tiletypes.hash.get(&position).unwrap() == &TileType::Wall {
                commands.entity(entity).despawn();
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
}