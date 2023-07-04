mod biome;
mod button_system;
mod click;
mod components;
mod constants;
mod game_ui;
mod input;
mod load;
mod main_menu;
mod map;
mod monstergenerator_system;
mod moverandom_system;
mod movetoward_system;
mod namegiving_system;
mod names_system;
mod needs;
mod pause;
mod prelude;
mod resources;
mod seasons;
mod selection_systems;
mod spoilage_system;
mod startup;
mod statusdisplay_system;
mod task_system;
mod text_system;
mod thinking_system;
mod window_system;

pub use prelude::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(biome::BiomePlugin)
        // Loading assets
        .add_startup_system_to_stage(StartupStage::PreStartup, load::load_sprites)
        .add_startup_system_to_stage(StartupStage::PreStartup, load::load_font)
        // fixing tick rates
        .add_fixed_timestep(Duration::from_millis(500), task_system::HALF_SECOND)
        .add_fixed_timestep(Duration::from_millis(2000), task_system::TWO_SECOND)
        .insert_resource(SelectedObjectInformation::default())
        .insert_resource(MenuState {
            state: MenuStates::Home,
        })
        // Map generation
        .add_plugin(startup::StartupPlugin)
        .add_startup_system(map::generate_map)
        // Display & menus
        .add_plugin(main_menu::MainMenusPlugin)
        .add_plugin(game_ui::GameUiPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(text_system::text_test)
        .add_startup_system(window_system::set_window_icon)
        .add_startup_system(window_system::set_window_maximized)
        .add_system(text_system::text_system)
        .add_system(text_system::text_update_system)
        // Game states
        .add_loopless_state(GameState::InGame)
        .add_state(GameState::InGame)
        .add_system(bevy::window::close_on_esc)
        .add_system_set(SystemSet::on_enter(GameState::Paused).with_system(pause::on_pause))
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(pause::on_unpause))
        // Input
        .add_plugin(button_system::ButtonPlugin)
        .add_plugin(selection_systems::SelectionPlugin)
        .add_plugin(click::ClickPlugin)
        .add_system(input::keyboard_input)
        .add_system(input::scrollwheel_input)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(moverandom_system::movement_random),
        )
        // NPC behaviour
        .add_plugin(task_system::TaskPlugin)
        .add_plugin(movetoward_system::MovementPlugin)
        .add_plugin(needs::NeedsPlugin)
        .add_system(movetoward_system::movement_toward_attackable)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(statusdisplay_system::status_display_system),
        )
        .add_plugin(thinking_system::ThinkingPlugin)
        // World simulation
        .add_plugin(spoilage_system::SpoilagePlugin)
        .add_plugin(seasons::SeasonsPlugin)
        .add_system(remove_bad_positions)
        .add_system(namegiving_system::namegiving_system)
        .add_system(names_system::names_system)
        .add_plugin(monstergenerator_system::MonsterGeneratorPlugin)
        .add_event::<needs::FoodNotifEvent>()
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
        if tiletypes.hash.contains_key(position) {
            if tiletypes.hash.get(position).unwrap().is_wall() {
                commands.entity(entity).despawn();
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
}
