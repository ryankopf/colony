use super::prelude::*;

pub fn nest_system(
    mut commands: Commands,
    query: Query<(Entity, &SetNest, &Position), Without<Nest>>,
) {
    for (entity, has_name, position) in query.iter() {
        commands.entity(entity).insert(Nest { position: position.clone() });
    }
}