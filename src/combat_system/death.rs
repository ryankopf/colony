use crate::prelude::*;

pub fn death_system(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Brain, &mut PhysicalBody, &Position), With<Dying>>,
) {
    for (entity, mut _brain, mut _physical_body, _position) in entities.iter_mut() {
        commands.entity(entity).despawn_recursive();
        // TO DO: Make a corpse and drop loot.
    }
}