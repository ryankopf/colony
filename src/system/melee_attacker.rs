use crate::prelude::*;


pub fn melee_attacks(
    mut commands: Commands,
    entities: Query<(Entity, &Position, With<MeleeAttacker>)>,
) {
    for (entity, position, _) in entities.iter() {

    }
}