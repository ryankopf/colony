use crate::prelude::*;

pub fn combat_system_melee(
    mut commands: Commands,
    mut entities_that_might_fight: Query<(Entity, &mut Brain, &Position, Option<&Pathing>, Option<&Targeting>)>,
    attackables: Query<(Entity, &Position), With<Attackable>>,
    sprite_sheet: Res<SpriteSheet>,
) {
    for (e, mut brain, position, pathing, targeting) in entities_that_might_fight.iter_mut() {
        if brain.task != Some(Task::Fight) { continue; }
        if let Some(targeting) = targeting {
            let mut entity_found = false;
            for (entity, target_position) in attackables.iter() {
                if entity == targeting.target {
                    if position.distance(target_position) <= 1 {
                        entity_found = true;
                        let sprite =  TextureAtlasSprite::new(StrikeType::Hit.sprite_index());
                        commands
                            .spawn(SpriteSheetBundle {
                                sprite,
                                texture_atlas: sprite_sheet.0.clone(),
                                ..default()
                            })
                            .insert(target_position.clone())
                            .insert(target_position.to_transform_layer(1.1))
                            ;
                        commands.entity(entity).insert(Attacked { attacker: e });
                        //do_melee_damage(&mut commands, entity, physical_body, &mut physical_body2);
                    } else {
                        // Try to follow/hunt the entity.
                        if pathing.is_none() {
                            commands.entity(e).insert( Pathing { path: vec![], destination: *target_position, ..default() });
                        }
                    }
                }
            }
            if !entity_found {
                commands.entity(e).remove::<Targeting>();
                brain.remotivate();
            }
        } else {
            // Find a target.
            // Humans might find a target based on if they're hunting or defending.
            // Animals might find a target based on if they're hungry or defending.
            // For now just find the nearest physical body and make that the target.
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
    attacker_entity: Option<Entity>,
    attacked_entity: Entity,
    body1: &PhysicalBody,
    body2: &mut PhysicalBody,
) {
    body2.attributes.health -= 10;
    println!("Health: {}", body2.attributes.health);
    if body2.attributes.health <= 0 {
        commands.entity(attacked_entity).despawn_recursive();
    }
    body2.danger = Some(Danger {
        danger_type: DangerType::Attacked,
        danger_source: attacker_entity,
    });
}
pub fn attacked_entities_system(
    mut commands: Commands,
    attacked_query: Query<(Entity, &Attacked), With<Attacked>>,
    mut physical_bodies: Query<(Entity, &mut PhysicalBody)>,
) {
    for (attacked_entity, attack_info) in attacked_query.iter() {
        println!("Attacked");
        commands.entity(attacked_entity).remove::<Attacked>();
        let mut attacker_physical_body: Option<PhysicalBody> = None;
        let mut attacker_entity = None;
        // Get the stats of the attacker.
        for (entity, physical_body) in physical_bodies.iter_mut() {
            if entity == attack_info.attacker {
                attacker_physical_body = Some(physical_body.clone());
                attacker_entity = Some(entity);
            }
        }
        if attacker_physical_body.is_none() { continue; }
        let attacker_physical_body = attacker_physical_body.unwrap();
        // Now do the damage to the attacked body.
        for (entity, mut physical_body) in physical_bodies.iter_mut() {
            if entity == attacked_entity {
                println!("Damage");
                do_melee_damage(&mut commands, attacker_entity, attacked_entity, &attacker_physical_body, &mut physical_body);
            }
        }
    }
}
