use crate::prelude::*;

pub fn task_system_forage(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain, &Position, Option<&Targeting>), Without<Pathing>>,
    mut foragables: Query<(Entity, &Position, &Foragable, &mut Plant)>,
    sprite_sheet: Res<SpriteSheet>,
) {
    let mut already_targeted = crate::set_already_targetted(&query);
    for (entity, mut brain, position, targeting) in query.iter_mut() {
        if brain.task != Some(Task::Forage) { continue; }
        let mut did_foraging = false;
        let mut shortest_distance = -1;
        let mut closest_entity = None;
        let mut closest_position = None;
        for (foragable_entity, foragable_position, _, mut plant) in foragables.iter_mut() {
            // If you are already next to it, forage it, if you are targetting it.
            let distance = position.distance(foragable_position);
            if distance <= 1 && targeting.is_some() && targeting.unwrap().target == foragable_entity {
                commands.entity(entity).remove::<Targeting>();
                spawn_food(&mut commands, foragable_entity, foragable_position, &sprite_sheet, &mut plant);
                did_foraging = true;
                closest_entity = None; closest_position = None;
                break;
            }
            // Unless it is already targetted by someone other than you.
            if already_targeted.contains(&foragable_entity) { continue; }
            
            if shortest_distance == -1 || distance < shortest_distance {
                shortest_distance = distance;
                closest_entity = Some(foragable_entity);
                closest_position = Some(foragable_position);
            }
        }
        if let Some(closest_entity) = closest_entity {
            commands.entity(entity).insert(Targeting { target: closest_entity });
            commands.entity(entity).insert(Pathing { path: vec![], destination: closest_position.unwrap().clone() });
            already_targeted.push(closest_entity);
        } else { // Just foraged, or there was no foragable.
            commands.entity(entity).remove::<Targeting>();
            if did_foraging {
                if brain.motivation == Some(Motivation::Hunger) {
                    brain.task = Some(Task::Eat);
                } else {
                    brain.remotivate();
                }
            } else { // Did not forage and could not find anything to forage.
                brain.remotivate();
            }
        }
    }
}

fn spawn_food(
    commands: &mut Commands,
    foragable_entity: Entity,
    foragable_position: &Position,
    sprite_sheet: &Res<SpriteSheet>,
    plant: &mut Plant,
) {
    plant.growth = 0.1;
    commands.entity(foragable_entity).remove::<Foragable>();
    // SPAWN TWO FOOD.
    for i in 2..4 {
        let mut p = foragable_position.clone();
        p.x += if (i%2) == 0 { i/2 } else { -i/2 };
        p.y += if (i%2) == 0 { i/2 } else { -i/2 };
        let sprite =  TextureAtlasSprite::new(88);
        commands.spawn(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: sprite_sheet.0.clone(),
            ..Default::default()
        })
        .insert(Food { ..default() } )
        .insert(p)
        .insert(p.to_transform_layer(2.0))
        ;
    }
}