use bevy::prelude::*;
use rand::prelude::random;
use bevy::time::FixedTimestep;
mod startup;
use startup::startup;
mod map;
use map::*;
mod components;
use components::*;
mod constants;
use constants::*;
mod moverandom_system;
use moverandom_system::*;
mod input;
use input::*;
mod prelude;

fn main() {
    //println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(generate_map)
        .add_startup_system(startup)
        .add_startup_system(setup_camera)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(movement_random),
        )
        .add_system(keyboard_input)
        .add_system(scrollwheel_input)
        .add_system(bevy::window::close_on_esc)
        .run();
        // .add_startup_system(setup.system())
        // .add_system(bevy::input::system::exit_on_esc_system.system())
        // .add_system(player_input.system())
        // .add_system(player_movement.system())
        // .add_system(monster_ai.system())
        // .add_system(map_indexing.system())
        // .add_system(viewshed_system.system())
        // .add_system(rendersystem.system())
        // .run();
}



fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn size_scaling(windows: Res<Windows>, mut q: Query<(&crate::components::SizeXYZ, &mut Transform)>) {
    if let Some(window) = windows.get_primary() {
        for (sprite_size, mut transform) in q.iter_mut() {
            transform.scale = Vec3::new(
                sprite_size.width / VIEWAREA_WIDTH as f32 * window.width() as f32,
                sprite_size.height / VIEWAREA_HEIGHT as f32 * window.height() as f32,
                1.0,
            );
        }
    }
}
fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    if let Some(window) = windows.get_primary() {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                convert(pos.x as f32, window.width() as f32, VIEWAREA_WIDTH as f32),
                convert(pos.y as f32, window.height() as f32, VIEWAREA_HEIGHT as f32),
                pos.z as f32,
            );
        }
    }
}