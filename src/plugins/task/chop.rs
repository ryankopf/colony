use crate::prelude::*;

pub fn task_system_chop(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain, &Position, Option<&Targeting>), Without<Pathing>>,
    mut targets: Query<(Entity, &Position, &Choppable, &mut Plant), With<WorkTarget>>,
    workmarkers: Query<(Entity, &Parent), With<WorkMarker>>,
    sprite_sheet: Res<SpriteSheet>,
) {
    let mut already_targeted = crate::plugin::task::set_already_targetted(&query);
    'outer: for (entity, mut brain, position, targeting) in query.iter_mut() {
        if brain.task != Some(Task::Chop) { continue; }
        let mut shortest_distance = -1;
        let mut closest_entity = None;
        let mut closest_position = None;
        for (targetable_entity, targetable_position, _, mut plant) in targets.iter_mut() {
            // If you are already next to it, chop it, if you are targetting it.
            let distance = position.distance(targetable_position);
            if distance <= 1 && targeting.is_some() && targeting.unwrap().target == targetable_entity {
                commands.entity(entity).remove::<Targeting>();
                crate::plugin::task::remove_x_markers(&mut commands, & workmarkers, targetable_entity);
                spawn_logs(&mut commands, targetable_entity, targetable_position, &sprite_sheet, &mut plant);
                continue 'outer;
            }
            // Unless it is already targetted by someone other than you.
            if already_targeted.contains(&targetable_entity) { continue; }

            if shortest_distance == -1 || distance < shortest_distance {
                shortest_distance = distance;
                closest_entity = Some(targetable_entity);
                closest_position = Some(targetable_position);
            }
        }
        if let Some(closest_entity) = closest_entity {
            commands.entity(entity).insert(Targeting { target: closest_entity });
            commands.entity(entity).insert(Pathing { path: vec![], destination: *closest_position.unwrap(), ..default() });
            already_targeted.push(closest_entity);
        } else {
            commands.entity(entity).remove::<Targeting>();
            commands.entity(entity).remove::<Pathing>();
            brain.remotivate();
        }
    }
}

// Note: In the future, drop seeds / pinecones / walnuts / etc.
fn spawn_logs(
    commands: &mut Commands,
    targetable_entity: Entity,
    targetable_position: &Position,
    sprite_sheet: &Res<SpriteSheet>,
    plant: &mut Plant,
) {
    //plant.growth = 0.1;
    let pt = plant.plant_type.is_choppable().0.unwrap_or( ItemType::PineLog );
    for i in 2..4 {
        commands.entity(targetable_entity).despawn(); // OR commands.entity(doable_entity).remove::<Choppable>();
        let mut p = *targetable_position;
        p.x += if (i%2) == 0 { i/2 } else { -i/2 };
        p.y += if (i%2) == 0 { i/2 } else { -i/2 };
        let sprite =  TextureAtlasSprite::new( pt.sprite_index() );
        commands.spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: sprite_sheet.0.clone(),
            ..Default::default()
        })
        .insert(Logs  )
        .insert(p)
        .insert(p.to_transform_layer(2.0))
        .insert( pt )
        ;
    }
}