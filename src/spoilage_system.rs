use super::prelude::*;

// Make Plugin
pub struct SpoilagePlugin;

impl Plugin for SpoilagePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(
            spoilage_system
            .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(2.0)))
            .run_if(in_state(GameState::InGame))
        )
        // .add_fixed_timestep_system(
        //     HALF_SECOND, 0,
        //     spoilage_system.run_in_bevy_state(GameState::InGame),
        // )
        ;
    }
}

pub fn spoilage_system(
    mut commands: Commands,
    mut food: Query<(Entity, &mut Food)>,
) {
    for (entity, mut food) in food.iter_mut() {
        food.spoilage -= food.spoilage_rate;
        if food.spoilage < 0.0 {
            // TO DO: ALERT PLAYER.
            commands.entity(entity).despawn();
        }
    }
}