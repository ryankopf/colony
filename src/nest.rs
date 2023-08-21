use super::prelude::*;

pub fn nest_system(
    mut commands: Commands,
    query: Query<(Entity, &Position), (Without<Nest>, With<SetNest>)>,
) {
    for (entity, position) in query.iter() {
        commands.entity(entity).insert(Nest { position: position.clone() });
    }
}