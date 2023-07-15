use std::time::Duration;

use bevy::time::FixedTimestep;
use plugin::biome::BiomePlugin;
use system::input::{keyboard_input, scrollwheel_input};
use crate::load::{load_font, load_sprites};
use crate::system::map::generate_map;
use plugin::monstergenerator::MonsterGeneratorPlugin;
use system::moverandom::movement_random;
use system::pause::{on_pause, on_unpause};
use crate::plugin::main_menu::MainMenusPlugin;

pub use crate::prelude::*;
use plugin::startup::StartupPlugin;
use crate::plugin::button::ButtonPlugin;
use crate::plugin::click::ClickPlugin;
use crate::plugin::game_ui::GameUiPlugin;
use crate::plugin::movetoward::{movement_toward_attackable, MovementPlugin};
use crate::plugin::needs::{FoodNotifEvent, NeedsPlugin};
use crate::plugin::seasons::SeasonsPlugin;
use crate::plugin::selection::SelectionPlugin;
use crate::plugin::spoilage::SpoilagePlugin;
use crate::plugin::task::TaskPlugin;
use crate::plugin::thinking::ThinkingPlugin;
use crate::system::namegiving::namegiving_system;
use crate::system::names::names_system;
use crate::system::statusdisplay::status_display_system;
use crate::system::text::{text_system, text_test, text_update_system};
use crate::system::window::{set_window_icon, set_window_maximized};

mod plugin;
mod system;

mod prelude;
mod components;
mod constants;
mod load;
mod resources;

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
