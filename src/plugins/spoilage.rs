use crate::plugins::task::HALF_SECOND;
use crate::prelude::*;

// Make Plugin
pub struct SpoilagePlugin;

impl Plugin for SpoilagePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_fixed_timestep_system(
            HALF_SECOND, 0,
            spoilage_system.run_in_bevy_state(GameState::InGame),
        )
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