use bevy::time::FixedTimestep;
mod prelude;
pub use crate::prelude::*;

use retrieve::mod_use;
use std::time::Duration;
#[mod_use(
    biome,
    button_system,
    click,
    components,
    constants,
    game_ui,
    input,
    load,
    main_menu,
    map,
    monstergenerator_system,
    moverandom_system,
    movetoward_system,
    namegiving_system,
    names_system,
    needs,
    pause,
    resources,
    seasons,
    selection_systems,
    spoilage_system,
    startup,
    statusdisplay_system,
    task_system,
    text_system,
    thinking_system,
    window_system
)]

fn main() {
    //println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BiomePlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_sprites)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_font)
        .add_fixed_timestep(Duration::from_millis(500), "half_second")
        .add_fixed_timestep(Duration::from_millis(2000), "two_second")
        .insert_resource(SelectedObjectInformation::default())
        .insert_resource(MenuState {
            state: MenuStates::Home,
        })
        .add_startup_system(generate_map)
        .add_plugin(StartupPlugin)
        .add_startup_system(setup_camera)
        .add_system(text_system)
        .add_startup_system(text_test)
        .add_startup_system(set_window_icon)
        .add_startup_system(set_window_maximized)
        .add_plugin(MainMenusPlugin)
        .add_state(GameState::InGame)
        .add_loopless_state(GameState::InGame)
        .add_plugin(ButtonPlugin)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(movement_random),
        )
        .add_plugin(SelectionPlugin)
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
        .add_plugin(SpoilagePlugin)
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
        .add_system_set(SystemSet::on_enter(GameState::Paused).with_system(on_pause))
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(on_unpause))
        .run();
}

// pub fn in_game(
//     state: Res<GameState>,
// ) {
//     *state == GameState::InGame
// }

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
        if tiletypes.hash.contains_key(position) {
            if tiletypes.hash.get(position).unwrap().is_wall() {
                commands.entity(entity).despawn();
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
}
