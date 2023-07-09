use super::prelude::*;
mod chop;
mod eat;
mod forage;
mod meander;
mod plant;
mod play;
mod sleep;
mod work;

pub const HALF_SECOND: &str = "half_second";
pub const TWO_SECOND: &str = "two_second";

// Make Plugin
pub struct TaskPlugin;

impl Plugin for TaskPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_timestep_system(HALF_SECOND, 0, eat::task_system_eat.run_in_bevy_state(GameState::InGame)).add_fixed_timestep_system(HALF_SECOND, 0, sleep::task_system_sleep.run_in_bevy_state(GameState::InGame)).add_fixed_timestep_system(HALF_SECOND, 0, sleep::task_system_sleeping.run_in_bevy_state(GameState::InGame)).add_fixed_timestep_system(HALF_SECOND, 0, play::task_system_playing.run_in_bevy_state(GameState::InGame)).add_fixed_timestep_system(HALF_SECOND, 0, meander::task_system_meander.run_in_bevy_state(GameState::InGame)).add_fixed_timestep_system(HALF_SECOND, 0, work::task_system_work.run_in_bevy_state(GameState::InGame)).add_fixed_timestep_system(HALF_SECOND, 0, forage::task_system_forage.run_in_bevy_state(GameState::InGame)).add_fixed_timestep_system(HALF_SECOND, 0, chop::task_system_chop.run_in_bevy_state(GameState::InGame)).add_fixed_timestep_system(HALF_SECOND, 0, plant::task_system_plant.run_in_bevy_state(GameState::InGame));
    }
}

pub fn set_already_targetted(query: &Query<(Entity, &mut Brain, &Position, Option<&Targeting>), Without<Pathing>>) -> Vec<Entity> {
    query.iter().filter(|(_, _, _, targeting)| targeting.is_some()).map(|(_, _, _, targeting)| targeting.unwrap().target).collect::<Vec<Entity>>()
}

pub fn remove_x_markers(commands: &mut Commands, workmarkers: &Query<(Entity, &Parent), With<WorkMarker>>, targetable_entity: Entity) {
    for (child, parent) in workmarkers.iter() {
        if parent.get() == targetable_entity {
            commands.entity(child).despawn();
        }
    }
}
