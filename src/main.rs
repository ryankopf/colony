use bevy::prelude::*;
use bevy::time::FixedTimestep;
mod startup;
use startup::*;
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
mod monstergenerator_system;
use monstergenerator_system::*;
mod movetoward_system;
use movetoward_system::*;
mod seasons;
use seasons::*;

fn main() {
    //println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(generate_map)
        .add_startup_system(startup)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_text)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(movement_random),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(monster_generator),
        )
        .add_system_set_to_stage(
            CoreStage::First,
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.05))
                .with_system(movement_along_path),
        )
        .add_system_set_to_stage(
            CoreStage::First,
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(movement_path_generating),
        )
        .add_system_set_to_stage(
            CoreStage::First,
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.0))
                .with_system(seasons),
        )
        .add_system(movement_toward_attackable)
        // .add_system(movement_path_generating)
        .add_system(keyboard_input)
        .add_system(scrollwheel_input)
        .add_system(bevy::window::close_on_esc)
        .run();
}


fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation.x = TILE_SIZE * 10.0;
    camera.transform.translation.y = TILE_SIZE * 10.0;
    commands.spawn(camera);

}