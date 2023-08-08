use crate::{prelude::*, initializations::load::SoundEffect};

pub fn combat_system_melee(
    mut commands: Commands,
    mut entities_that_might_fight: Query<(Entity, &mut Brain, &mut PhysicalBody, &Position, Option<&mut Pathing>, Option<&Targeting>)>,
    attackables: Query<(Entity, &Position), With<Brain>>,
    sprite_sheet: Res<SpriteSheet>,
) {
    for (e, mut brain, mut physical_body, position, pathing, targeting) in entities_that_might_fight.iter_mut() {
        if brain.task != Some(Task::Fight) { continue; }
        if let Some(targeting) = targeting {
            let mut entity_found = false;
            for (entity, target_position) in attackables.iter() {
                if entity == targeting.target {
                    entity_found = true;
                    if position.distance(target_position) <= 1 {
                        let sprite =  TextureAtlasSprite::new(StrikeType::Hit.sprite_index());
                        commands
                            .spawn(SpriteSheetBundle {
                                sprite,
                                texture_atlas: sprite_sheet.0.clone(),
                                ..default()
                            })
                            .insert(target_position.clone())
                            .insert(target_position.to_transform_layer(1.1))
                            .insert( TemporaryVisualElement { duration: 0.2 } )
                            ;
                        commands.entity(entity).insert(Attacked { attacker: e });
                        if pathing.is_some() { commands.entity(e).remove::<Pathing>(); }
                    } else {
                        // Try to follow/hunt the entity.
                        if pathing.is_none() {
                            commands.entity(e).insert( Pathing { path: vec![], destination: *target_position, ..default() });
                        } else {
                            let mut path = pathing.unwrap();
                            // path.destination = *target_position;
                            path.moving_target = true;
                            //path.path = vec![];
                        }
                    }
                    break;
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
            if let Some(danger) = &physical_body.danger {
                if danger.danger_type == DangerType::Attacked {
                    // If we're being attacked, we should attack back.
                    // Error: What happens after you win the fight? Or if the attacker no longer exists?
                    if let Some(danger_source) = danger.danger_source {
                        // check if attackables contains danger_source
                        let mut danger_source_found = false;
                        for (entity, _target_position) in attackables.iter() {
                            if entity == danger_source {
                                danger_source_found = true;
                                break;
                            }
                        }
                        if !danger_source_found {
                            // The danger source no longer exists. We should stop attacking.
                            brain.remotivate();
                            physical_body.danger = None;
                            continue;
                        }
                        commands.entity(e).insert(Targeting { target: danger_source });
                        continue;
                    }
                }
            }
            let mut closest_distance = 9999;
            let mut closest_target = None;
            let mut closest_position = None;
            for (attackable, attackable_position) in attackables.iter() {
                if attackable == e { continue; }
                let distance = position.distance(attackable_position);
                if distance < closest_distance {
                    closest_distance = distance;
                    closest_target = Some(attackable);
                    closest_position = Some(attackable_position);
                }
            }
            if let Some(closest_target) = closest_target {
                commands.entity(e).insert(Targeting { target: closest_target });
                let target_position = closest_position.unwrap();
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
    asset_server: &Res<AssetServer>
) {
    let damage =
        1 +
        (body1.attributes.strength - body2.attributes.constitution).max(0).min(20) +
        (body1.skillset.brawling.level()).max(0).min(20)
        ;
    body2.attributes.health -= damage;
    if body2.attributes.health <= 0 {
        commands.entity(attacked_entity).despawn_recursive();
    }
    body2.danger = Some(Danger {
        danger_type: DangerType::Attacked,
        danger_source: attacker_entity,
    });

    // Play a sound effect.
    commands.spawn((
        AudioBundle {
            source: asset_server.load("RPG Sound Pack/battle/swing.wav"),
            settings: PlaybackSettings::ONCE.with_volume(bevy::audio::Volume::new_relative(0.1)),
        },
        SoundEffect,
    ));
}
pub fn attacked_entities_system(
    mut commands: Commands,
    attacked_query: Query<(Entity, &Attacked), With<Attacked>>,
    mut physical_bodies: Query<(Entity, &mut PhysicalBody)>,
    asset_server: Res<AssetServer>
) {
    for (attacked_entity, attack_info) in attacked_query.iter() {
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
                do_melee_damage(&mut commands, attacker_entity, attacked_entity, &attacker_physical_body, &mut physical_body, &asset_server);
            }
        }
    }
}

pub fn temporary_visual_elements_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TemporaryVisualElement)>,
) {
    let delta_seconds = time.delta_seconds();
    for (entity, mut tve) in query.iter_mut() {
        tve.duration -= delta_seconds;
        if tve.duration <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
