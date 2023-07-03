use crate::prelude::*;

pub fn task_system_plant(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain, &Position, Option<&Targeting>), Without<Pathing>>,
    targetables: Query<(Entity, &Position, &Zone)>,
    // obstacles: Query<&Position, (Without<Brain>, Without<MapTile>)>, // This seems to be an "AND"
    obstacles: Query<(Entity, &Position), Without<MapTile>>,
    sprite_sheet: Res<SpriteSheet>,
) {
    let mut already_targeted = crate::set_already_targetted(&query);
    'brains: for (entity, mut brain, position, targeting) in query.iter_mut() {
        if brain.task != Some(Task::Plant) { continue; }
        let mut nearest_entity: Option<NearestEntity> = None;
        'targets: for (targetable_entity, targetable_position, zone) in targetables.iter() {
            if zone.zone_type != ZoneType::Farm { continue; }
            for (e, obstacle) in obstacles.iter() {
                if (obstacle == targetable_position) && (entity != e) { continue 'targets; }
            } // Don't plant on top of obstacles.
            // If you are already next to it, plant it, if you are targetting it.
            let distance = position.distance(targetable_position);
            if distance <= 1 && targeting.is_some() && targeting.unwrap().target == targetable_entity {
                commands.entity(entity).remove::<Targeting>();
                spawn_plant(&mut commands, targetable_position, &sprite_sheet, zone); // Did plant! Now, go ahead and try planting again....
                continue 'brains;
            }
            // Unless it is already targetted by someone other than you.
            if already_targeted.contains(&targetable_entity) { continue; }
              
            if nearest_entity.is_none() || distance < nearest_entity.as_ref().unwrap().distance {
                nearest_entity = Some(NearestEntity { entity: targetable_entity, distance, position: *targetable_position })
            }
        }
        if let Some(nearest_entity) = nearest_entity {
            commands.entity(entity).insert(Targeting { target: nearest_entity.entity });
            commands.entity(entity).insert(Pathing { path: vec![], destination: nearest_entity.position, ..default() });
            already_targeted.push(nearest_entity.entity);
        } else {
            commands.entity(entity).remove::<Targeting>();
            brain.remotivate();
        }
    }
}

fn spawn_plant(
    commands: &mut Commands,
    position: &Position,
    sprite_sheet: &Res<SpriteSheet>,
    zone: &Zone,
) {
    // commands.entity(foragable_entity).remove::<Foragable>();
    let sprite =  TextureAtlasSprite::new(zone.plant_type.sprite_index());
    commands.spawn(SpriteSheetBundle {
        sprite,
        texture_atlas: sprite_sheet.0.clone(),
        transform: Transform::from_xyz(
            position.x as f32 * TILE_SIZE,
            position.y as f32 * TILE_SIZE,
            position.z as f32 * TILE_SIZE,
        ),
        ..default()
    })
    .insert(*position)
    .insert(position.to_transform_layer(0.5))
    .insert(Plant { growth: 0.4, plant_type: zone.plant_type })
    ;
}