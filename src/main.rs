mod prelude;
pub use crate::prelude::*;

mod button_system;
use button_system::*;
mod combat_system;
use combat_system::*;
mod components;
mod constants;
mod initializations;
use initializations::*;
mod interface;
use interface::*;
mod objects;
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
mod nest;
use nest::*;
mod resources;
mod seasons;
use seasons::*;
mod selection_systems;
use selection_systems::*;
mod spoilage_system;
use spoilage_system::*;
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

fn main() {
    //println!("Hello, world!");
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins((DefaultPlugins, BiomePlugin, StartupPlugin))
        .add_systems(
            PreStartup, (load_sprites, load_font, load_sfx)
        )
        .insert_resource(SelectedObjectInformation::default())
        .insert_resource(InfoPanelInformation::default())
        .insert_resource(MenuState {
            state: MenuStates::Home,
        })
        .add_systems(
            Startup, (generate_map, setup_camera, text_test, set_window_title, set_window_icon, set_window_maximized)
        )
        .add_state::<GameState>()
        .add_plugins((MainMenusPlugin, ButtonPlugin))
        .add_systems(
            Update,
            movement_random
            .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.1)))
            .run_if(in_state(GameState::InGame))
        )
        .add_plugins(
            (SelectionPlugin, MonsterGeneratorPlugin, MovementPlugin, SeasonsPlugin, NeedsPlugin, GameUiPlugin,
                InfoPanelPlugin, ThinkingPlugin, TaskPlugin, CombatPlugin, SpoilagePlugin, ClickPlugin))
        .add_systems(
            Update,
            status_display_system
            .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
            .run_if(in_state(GameState::InGame))
        )
        .add_systems(Update, (
            text_system,
            remove_bad_positions,
            namegiving_system,
            names_system,
            text_update_system,
            movement_toward_attackable,
            keyboard_input,
            scrollwheel_input,
            nest_system,
        ))
        .add_event::<FoodNotifEvent>()
        .add_systems(
            Update,
            bevy::window::close_on_esc
        )
        .add_systems(
            OnEnter(GameState::Paused), 
            on_pause
        )
        .add_systems(
            OnExit(GameState::Paused), 
            on_unpause
        )
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
