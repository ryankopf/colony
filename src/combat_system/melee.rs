use crate::prelude::*;

pub fn combat_system_melee(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain, &Position, Option<&Targeting>)>,
    attackables: Query<(Entity, &Position), With<Attackable>>,
    sprite_sheet: Res<SpriteSheet>,
) {
    for (e, mut brain, position, targeting) in query.iter_mut() {
        if brain.task != Some(Task::Fight) { continue; }
        if let Some(targeting) = targeting {
            let mut entity_found = false;
            for (entity, position2) in attackables.iter() {
                if entity == targeting.target {
                    if position.distance(position2) <= 1 {
                        println!("Attack!");
                        entity_found = true;
                        let sprite =  TextureAtlasSprite::new(StrikeType::Hit.sprite_index());
                        commands
                            .spawn(SpriteSheetBundle {
                                sprite,
                                texture_atlas: sprite_sheet.0.clone(),
                                ..default()
                            })
                            .insert(position2.clone());
                        //commands.entity(entity).insert(Attacked { attacker: e });
                        //do_melee_damage(&mut commands, entity, physical_body, &mut physical_body2);
                    } else {
                        // Try to follow/hunt the entity.
                    }
                }
            }
            if !entity_found {
                brain.motivation = None;
                brain.task = None;
            }
        } else {
            // Find a target.
            // Humans might find a target based on if they're hunting or defending.
            // Animals might find a target based on if they're hungry or defending.
            // For now just find the nearest physical body and make that the target.
            println!("Find a target.");
            let mut closest_distance = 9999;
            let mut closest_target = None;
            for (attackable, attackable_position) in attackables.iter() {
                let distance = position.distance(attackable_position);
                if distance < closest_distance {
                    closest_distance = distance;
                    closest_target = Some(attackable);
                }
            }
            if let Some(closest_target) = closest_target {
                commands.entity(e).insert(Targeting { target: closest_target });
                let target_position = attackables.get(closest_target).unwrap().1;
                commands.entity(e).insert( Pathing { path: vec![], destination: *target_position, ..default() });
            } else {
                // Nothing to attack. Now what?
                brain.remotivate();
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
    if body2.attributes.health <= 0 {
        commands.entity(entity).despawn_recursive();
    }
}
