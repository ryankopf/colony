use super::prelude::*;

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