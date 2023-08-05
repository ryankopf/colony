use crate::prelude::*;

pub fn combat_system_melee(
    mut commands: Commands,
    mut query: Query<(&mut Brain, &PhysicalBody, &Position, Option<&Targeting>)>,
    mut positions: Query<(Entity, &Position, &mut PhysicalBody)>,
) {
    for (mut brain, physical_body, position, targeting) in query.iter_mut() {
        if brain.task != Some(Task::Fight) { continue; }
        if let Some(targeting) = targeting {
            let mut entity_found = false;
            for (entity, position2, mut physical_body2) in positions.iter_mut() {
                if entity == targeting.target {
                    if position.distance(position2) <= 1 {
                        println!("Attack!");
                        entity_found = true;
                        do_melee_damage(&mut commands, entity, physical_body, &mut physical_body2);
                    } else {
                        // Try to follow/hunt the entity.
                    }
                }
            }
            if !entity_found {
                brain.motivation = None;
                brain.task = None;
            }
        }
    }
}

fn do_melee_damage(
    commands: &mut Commands,
    entity: Entity,
    body1: &PhysicalBody,
    body2: &mut PhysicalBody,
) {
    body2.attributes.health -= 10;
    body2.danger = Some(Danger::Attacked);
    if body2.attributes.health <= 0 {
        commands.entity(entity).despawn_recursive();
    }
}