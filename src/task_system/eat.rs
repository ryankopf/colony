use crate::prelude::*;

pub fn task_system_eat(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain, &Position, Option<&Targeting>, Option<&mut Status>), Without<Pathing>>,
    mut query_food: Query<(Entity, &Position, &Food)>,
) {
    // Set list of entities that are already being targetted.
    let mut already_targeted = query.iter().filter(|(_, _, _, targeting, _)| targeting.is_some()).map(|(_, _, _, targeting, _)| targeting.unwrap().target).collect::<Vec<Entity>>();
    for (entity, mut brain, position, targeting, mut status) in query.iter_mut() {
        if brain.task != Some(Task::Eat) { continue; }
        // Get nearest food.
        // Set that as your target.
        // Move towards.
        let mut found_food = false;
        let mut shortest_distance = -1;
        let mut closest_entity = None;
        let mut closest_position = None;
        for (food_entity, food_position, _) in query_food.iter() { // Future food nutrition & distance calculate.
            if already_targeted.contains(&food_entity) && (targeting.is_none() || (targeting.is_some() && targeting.unwrap().target != food_entity)) { continue; }
            let distance = position.distance(food_position);
            if distance <= 1 && targeting.is_some() && targeting.unwrap().target == food_entity {
                // Eat it now.
                // Heal your status.
                if let Some(s) = status.as_mut() {
                    if let Some(n) = s.needs_food.as_mut() {
                        n.current = n.max;
                    }
                }
                // Remove the food.
                commands.entity(food_entity).despawn();
                // Remove the targeting.
                commands.entity(entity).remove::<Targeting>();
                if brain.motivation == Some(Motivation::Hunger) { brain.remotivate(); } // You're done!!
                closest_entity = None; closest_position = None;
                found_food = true;
                break;
            }
            if shortest_distance == -1 || distance < shortest_distance {
                shortest_distance = distance;
                closest_entity = Some(food_entity);
                closest_position = Some(food_position);
            }
        }
        if let Some(closest_entity) = closest_entity {
            commands.entity(entity).insert(Targeting { target: closest_entity });
            commands.entity(entity).insert(Pathing { path: vec![], destination: closest_position.unwrap().clone(), ..default() });
            already_targeted.push(closest_entity);
            found_food = true;
        } else {
            commands.entity(entity).remove::<Targeting>();
        }
        if !found_food {
            if let Some(Motivation::Hunger) = brain.motivation {
                brain.task = Some(Task::Forage);
            } else {
                brain.task = None;
            }
            // Set task to Forage or Hunt
            //commands.entity(entity).insert(Task::None);
        }
    }
}