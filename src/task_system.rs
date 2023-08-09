use super::prelude::*;
mod chop;
mod eat;
mod forage;
mod meander;
mod personality;
use personality::PersonalityPlugin;
mod plant;
mod play;
mod sleep;
mod work;

// Make Plugin
pub struct TaskPlugin;

impl Plugin for TaskPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(PersonalityPlugin)
        .add_systems(
            Update,
            (
                eat::task_system_eat
                .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
                .run_if(in_state(GameState::InGame))
            ,
                sleep::task_system_sleep
                .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
                .run_if(in_state(GameState::InGame))
            ,
                sleep::task_system_sleeping
                .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
                .run_if(in_state(GameState::InGame))
            ,
                play::task_system_playing
                .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
                .run_if(in_state(GameState::InGame))
            ,
                meander::task_system_meander
                .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
                .run_if(in_state(GameState::InGame))
            ,
                work::task_system_work
                .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
                .run_if(in_state(GameState::InGame))
            ,
                forage::task_system_forage
                .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
                .run_if(in_state(GameState::InGame))
            ,
                chop::task_system_chop
                .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
                .run_if(in_state(GameState::InGame))
            ,
                plant::task_system_plant
                .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
                .run_if(in_state(GameState::InGame))
            )
        )
        ;
    }
}

pub fn set_already_targetted(
    entities_that_might_target_things: &Query<(Entity, &mut Brain, &Position, Option<&Pathing>, Option<&Targeting>)>
) -> Vec<Entity> {
    entities_that_might_target_things
        .iter()
        .filter(|(_, _, _, _, targeting)| targeting.is_some())
        .map(|(_, _, _, _, targeting)| targeting.unwrap().target)
        .collect::<Vec<Entity>>()
}

pub fn remove_x_markers(
    commands: &mut Commands,
    workmarkers: &Query<(Entity, &Parent), With<WorkMarker>>,
    targetable_entity: Entity,
) {
    for (child, parent) in workmarkers.iter() {
        if parent.get() == targetable_entity {
            commands.entity(child).despawn();
        }
    }
}
