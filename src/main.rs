// use bevy::time::FixedTimestep;
mod prelude;
pub use crate::prelude::*;

use retrieve::mod_use;
use std::time::Duration;

// #[mod_use(
//     biome,
//     button_system,
//     combat_system,
//     components,
//     constants,
//     interface,
//     load,
//     main_menu,
//     map,
//     monstergenerator_system,
//     moverandom_system,
//     movetoward_system,
//     namegiving_system,
//     names_system,
//     needs,
//     pause,
//     resources,
//     seasons,
//     selection_systems,
//     spoilage_system,
//     startup,
//     statusdisplay_system,
//     task_system,
//     text_system,
//     thinking_system,
//     unitgenerator_system,
//     window_system
// )]
mod biome;
use biome::*;
mod button_system;
use button_system::*;
mod combat_system;
use combat_system::*;
mod components;
use components::*;
mod constants;
use constants::*;
mod interface;
use interface::*;
mod load;
use load::*;
mod main_menu;
use main_menu::*;
mod map;
use map::*;
mod monstergenerator_system;
use monstergenerator_system::*;
mod moverandom_system;
use moverandom_system::*;
mod movetoward_system;
use movetoward_system::*;
mod namegiving_system;
use namegiving_system::*;
mod names_system;
use names_system::*;
mod needs;
use needs::*;
mod pause;
use pause::*;
mod resources;
use resources::*;
mod seasons;
use seasons::*;
mod selection_systems;
use selection_systems::*;
mod spoilage_system;
use spoilage_system::*;
mod startup;
use startup::*;
mod statusdisplay_system;
use statusdisplay_system::*;
mod task_system;
use task_system::*;
mod text_system;
use text_system::*;
mod thinking_system;
use thinking_system::*;
mod unitgenerator_system;
use unitgenerator_system::*;
mod window_system;
use window_system::*;

fn main() {
    //println!("Hello, world!");
    App::new()
        //.insert_resource(FixedTime::new(Duration::from_millis(100)))
        .add_plugins(DefaultPlugins)
        .add_plugin(BiomePlugin)
        .add_systems(
            PreStartup, (load_sprites, load_font)
        )
        .add_systems(
            PreStartup, (load_sprites, load_font)
        )
        // .add_startup_system_to_stage(StartupStage::PreStartup, load_sprites)
        // .add_startup_system_to_stage(StartupStage::PreStartup, load_font)
        // .add_fixed_timestep(Duration::from_millis(500), HALF_SECOND)
        // .add_fixed_timestep(Duration::from_millis(2000), TWO_SECOND)
        .insert_resource(SelectedObjectInformation::default())
        .insert_resource(InfoPanelInformation::default())
        .insert_resource(MenuState {
            state: MenuStates::Home,
        })
        .add_systems(
            Startup, (generate_map, setup_camera, text_test, set_window_icon, set_window_maximized)
        )
        // .add_startup_system(generate_map)
        .add_plugin(StartupPlugin)
        // .add_startup_system(setup_camera)
        .add_system(text_system)
        // .add_startup_system(text_test)
        // .add_startup_system(set_window_icon)
        // .add_startup_system(set_window_maximized)
        .add_plugin(MainMenusPlugin)
        .add_state::<GameState>()
        //.add_state(GameState::InGame)
        // .add_loopless_state(GameState::InGame)
        .add_plugin(ButtonPlugin)
        .add_system(
            movement_random
            .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.1)))
            .run_if(in_state(GameState::InGame))
        )
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(0.1))
        //         .with_system(movement_random),
        // )
        .add_plugin(SelectionPlugin)
        .add_plugin(MonsterGeneratorPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(SeasonsPlugin)
        .add_plugin(NeedsPlugin)
        .add_system(
            status_display_system
            .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
            .run_if(in_state(GameState::InGame))
        )
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(0.5))
        //         .with_system(status_display_system),
        // )
        .add_plugin(GameUiPlugin)
        .add_plugin(InfoPanelPlugin)
        .add_plugin(ThinkingPlugin)
        .add_plugin(TaskPlugin)
        .add_plugin(CombatPlugin)
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
        .add_systems(
            OnEnter(GameState::Paused), 
            on_pause
        )
        .add_systems(
            OnExit(GameState::Paused), 
            on_unpause
        )
        // .add_system_set(SystemSet::on_enter(GameState::Paused).with_system(on_pause))
        // .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(on_unpause))
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
