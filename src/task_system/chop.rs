use crate::prelude::*;

pub fn task_system_chop(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain, &Position, Option<&Targeting>), Without<Pathing>>,
    mut targets: Query<(Entity, &Position, &Choppable, &mut Plant), With<WorkTarget>>,
    mut workmarkers: Query<(Entity, &Parent), With<WorkMarker>>,
    sprite_sheet: Res<SpriteSheet>,
) {
    let mut already_targeted = query.iter().filter(|(_, _, _, targeting)| targeting.is_some()).map(|(_, _, _, targeting)| targeting.unwrap().target).collect::<Vec<Entity>>();
    for (entity, mut brain, position, targeting) in query.iter_mut() {
        if brain.task != Some(Task::Chop) { continue; }
        let mut shortest_distance = -1;
        let mut closest_entity = None;
        let mut closest_position = None;
        for (doable_entity, doable_position, _, mut plant) in targets.iter_mut() {
            // Unless it is already targetted by someone other than you.
            if already_targeted.contains(&doable_entity) && (targeting.is_none() || (targeting.is_some() && targeting.unwrap().target != doable_entity)) { continue; }
            let distance = position.distance(doable_position);
            if distance <= 1 && targeting.is_some() && targeting.unwrap().target == doable_entity {
                plant.growth = 0.1;
                for (child, parent) in workmarkers.iter_mut() {
                    if parent.get() == doable_entity {
                        commands.entity(child).despawn();
                    }
                }
                commands.entity(doable_entity).despawn();
                // commands.entity(doable_entity).remove::<Choppable>();
                commands.entity(entity).remove::<Targeting>();
                // SPAWN TWO LOGS.
                for i in 2..4 {
                    let mut p = doable_position.clone();
                    p.x += if (i%2) == 0 { (i/2) } else { -(i/2) };
                    p.y += if (i%2) == 0 { (i/2) } else { -(i/2) };
                    let sprite =  TextureAtlasSprite::new(SPRITES::LOGS as usize);
                    commands.spawn(SpriteSheetBundle {
                        sprite: sprite,
                        texture_atlas: sprite_sheet.0.clone(),
                        transform: Transform::from_xyz(
                            position.x as f32 * TILE_SIZE,
                            position.y as f32 * TILE_SIZE,
                            position.z as f32 * TILE_SIZE,
                        ),
                        ..Default::default()
                    })
                    .insert(Logs { ..default() } )
                    .insert(p)
                    .insert(p.to_transform_layer(2.0))
                    ;
                }
                closest_entity = None; closest_position = None;
                break;
            }
            if shortest_distance == -1 || distance < shortest_distance {
                shortest_distance = distance;
                closest_entity = Some(doable_entity);
                closest_position = Some(doable_position);
            }
        }
        if let Some(closest_entity) = closest_entity {
            commands.entity(entity).insert(Targeting { target: closest_entity });
            commands.entity(entity).insert(Pathing { path: vec![], destination: closest_position.unwrap().clone() });
            already_targeted.push(closest_entity);
        } else {
            commands.entity(entity).remove::<Targeting>();
            commands.entity(entity).remove::<Pathing>();
            brain.remotivate();
        }
    }
}