use super::prelude::*;

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
                if brain.motivation == Some(Motivation::Hunger) { brain.motivation = None; brain.task = None; } // You're done!!
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
            commands.entity(entity).insert(Pathing { path: vec![], destination: closest_position.unwrap().clone() });
            already_targeted.push(closest_entity);
            found_food = true;
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

pub fn task_system_sleep(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain, &Position), Without<Targeting>>,
    mut query_bed: Query<(Entity, &Position, &Bed)>,
) {
    for (entity, mut brain, position) in query.iter_mut() {
        if let None = &brain.task {
            continue; // Has no task.
        }
        let task = brain.task.unwrap();
        if task != Task::Sleep { continue; }
        // Get nearest bed.
        // Set that as your target.
        // Move towards.
        let mut found_bed = false;
        let mut shortest_distance = -1;
        for (_, bed_position, _) in query_bed.iter() { // Future food nutrition & distance calculate.
            let distance = position.distance(bed_position);
            if shortest_distance == -1 || distance < shortest_distance {
                shortest_distance = distance;
            }
        }
        for (bed_entity, bed_position, _) in query_bed.iter_mut() { // Future food nutrition & distance calculate.
            let distance = position.distance(bed_position);
            if distance == shortest_distance {
                // Set target.
                commands.entity(entity).insert(Targeting { target: bed_entity });
                commands.entity(entity).insert(Pathing { path: vec![], destination: bed_position.clone() });
                found_bed = true;
                break;
            }
        }
        if !found_bed {
            brain.task = Some(Task::Sleeping);
        }
    }
}

pub fn task_system_work(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain, &Position), Without<Targeting>>
) {
    for (entity, mut brain, position) in query.iter_mut() {
        // There is no work right now.
        if brain.task != Some(Task::Work) { continue; }
        let mut rng = rand::thread_rng();
        // Generate number between 0 and 10
        let number: u8 = rng.gen_range(0..10);
        match number {
            0 => brain.task = Some(Task::Forage),
            // 1 => brain.task = Some(Task::Eat),
            // 2 => brain.task = Some(Task::Work),
            // 3 => brain.task = Some(Task::Forage),
            // 4 => brain.task = Some(Task::Hunt),
            // 5 => brain.task = Some(Task::Sleep),
            // 6 => brain.task = Some(Task::Eat),
            // 7 => brain.task = Some(Task::Work),
            // 8 => brain.task = Some(Task::Forage),
            // 9 => brain.task = Some(Task::Hunt),
            _ => brain.task = Some(Task::Meander),
        }
    }
}

pub fn task_system_meander(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain, &mut Position, &mut Transform), Without<TileType>>,
    tile_types: Query<(&Position, &TileType)>,
) {
    for (entity, mut brain, mut position, mut transform) in query.iter_mut() {
        if brain.task != Some(Task::Meander) { continue; }
        brain.task = Some(Task::Meander);
        let mut new_position = *position;
        let mut rng = rand::thread_rng();
        let dir = rng.gen_range(0..4);
        match dir {
            0 => new_position.y += 1,
            1 => new_position.y -= 1,
            2 => new_position.x -= 1,
            3 => new_position.x += 1,
            _ => {}
        }
        for (tile_position, tile_type) in tile_types.iter() {
            let mut p2 = new_position;
            p2.z = 0;
            if *tile_position == p2 {
                if *tile_type != TileType::Wall {
                    *position = new_position;
                    transform.translation.x = new_position.x as f32 * TILE_SIZE;
                    transform.translation.y = new_position.y as f32 * TILE_SIZE;
                }
            }
        }
    }
}
pub fn task_system_sleeping(
    mut commands: Commands,
    mut query: Query<(&mut Brain, &mut Status)>
) {
    for (mut brain, mut status) in query.iter_mut() {
        // There is no work right now.
        if (brain.task != Some(Task::Sleeping)) { continue; }
        if let Some(n) = &mut status.needs_sleep {
            n.current += 10.0;
            if n.current >= n.max {
                brain.motivation = None;
                brain.task = None;
            }
        }
    }
}

pub fn task_system_forage(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain, &Position, Option<&Targeting>), Without<Pathing>>,
    mut foragables: Query<(Entity, &Position, &Foragable, &mut Plant)>,
) {
    let mut already_targeted = query.iter().filter(|(_, _, _, targeting)| targeting.is_some()).map(|(_, _, _, targeting)| targeting.unwrap().target).collect::<Vec<Entity>>();
    for (entity, mut brain, position, targeting) in query.iter_mut() {
        if brain.task != Some(Task::Forage) { continue; }
        let mut did_foraging = false;
        let mut shortest_distance = -1;
        let mut closest_entity = None;
        let mut closest_position = None;
        for (foragable_entity, foragable_position, _, mut plant) in foragables.iter_mut() {
            // Unless it is already targetted by someone other than you.
            if already_targeted.contains(&foragable_entity) && (targeting.is_none() || (targeting.is_some() && targeting.unwrap().target != foragable_entity)) { continue; }
            let distance = position.distance(foragable_position);
            if distance <= 1 && targeting.is_some() && targeting.unwrap().target == foragable_entity {
                plant.growth = 0.1;
                commands.entity(foragable_entity).remove::<Foragable>();
                commands.entity(entity).remove::<Targeting>();
                // SPAWN TWO FOOD.
                for i in 2..4 {
                    let mut p = foragable_position.clone();
                    p.x += if (i%2) == 0 { (i/2) } else { -(i/2) };
                    p.y += if (i%2) == 0 { (i/2) } else { -(i/2) };
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::ANTIQUE_WHITE,
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Food { ..default() } )
                    .insert(p)
                    .insert(p.to_transform_layer(2.0))
                    ;
                }
                did_foraging = true;
                closest_entity = None; closest_position = None;
                break;
            }
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
            if did_foraging {
                if brain.motivation == Some(Motivation::Hunger) {
                    brain.task = Some(Task::Eat);
                } else {
                    brain.remotivate();
                }
            } else { // Did not forage and could not find anything to forage.
                //brain.task = Some(Task::Meander);
                brain.remotivate();
            }
        }
    }
}

// if task == "Eat" {
//     commands.entity(entity).insert(TaskEat {});
// } else if task == "Sleep" {
//     commands.entity(entity).insert(TaskSleep {});
// } else if task == "Entertain" {
//     commands.entity(entity).insert(TaskEntertain {});
// } else if task == "Work" {
//     commands.entity(entity).insert(TaskWork {});
// } else if task == "Socialize" {
//     commands.entity(entity).insert(TaskSocialize {});
// } else if task == "Wander" {
//     commands.entity(entity).insert(TaskWander {});
// } else if task == "Crisis" {
//     commands.entity(entity).insert(TaskCrisis {});
// } else if task == "Danger" {
//     commands.entity(entity).insert(TaskDanger {});
// } else if task == "Order" {
//     commands.entity(entity).insert(TaskOrder {});
// } else if task == "Hospital" {
//     commands.entity(entity).insert(TaskHospital {});
// } else if task == "Repair" {
//     commands.entity(entity).insert(TaskRepair {});
// } else if task == "Build" {
//     commands.entity(entity).insert(TaskBuild {});
// } else if task == "Mine" {
//     commands.entity(entity).insert(TaskMine {});
// } else if task == "Farm" {
//     commands.entity(entity).insert(TaskFarm {});
// } else if task == "Hunt" {
//     commands.entity(entity).insert(TaskHunt {});
// } else if task == "Fish" {
//     commands.entity(entity).insert(TaskFish {});
// } else if task == "Gather" {
//     commands.entity(entity).insert(TaskGather {});
// } else if task == "Forage" {
//     commands.entity(entity).insert(TaskForage {});
// } else if task == "Chop" {
//     commands.entity(entity).insert(TaskChop {});
// } else if task == "Cut" {
//     commands.entity(entity).insert(TaskCut {});
// } else if task == "Dig" {
//     commands.entity(entity).insert(TaskDig {});
// } else if task == "Clean" {
//     commands.entity(entity).insert(TaskClean {});
// } else if task == "Cook" {
//     commands.entity(entity).insert(TaskCook {});
// } else if task == "Craft" {
//     commands.entity(entity).insert(TaskCraft {});
// } else if task == "Research" {
//     commands.entity(entity).insert(TaskResearch {});
// }
