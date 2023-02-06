use super::prelude::*;

pub fn task_system_eat(
    mut commands: Commands,
    query: Query<(Entity, &Brain, &Position), Without<Targeting>>,
    mut query_food: Query<(Entity, &Position, &Food)>,
) {
    for (entity, brain, position) in query.iter() {
        if let None = &brain.task {
            continue; // Has no task.
        }
        let task = brain.task.unwrap();
        if task != Task::Eat { continue; }
        // Get nearest food.
        // Set that as your target.
        // Move towards.
        let mut found_food = false;
        let mut shortest_distance = -1;
        for (_, food_position, _) in query_food.iter() { // Future food nutrition & distance calculate.
            let distance = position.distance(food_position);
            if shortest_distance == -1 || distance < shortest_distance {
                shortest_distance = distance;
            }
        }
        for (food_entity, food_position, _) in query_food.iter_mut() { // Future food nutrition & distance calculate.
            let distance = position.distance(food_position);
            if distance == shortest_distance {
                // Set target.
                commands.entity(entity).insert(Targeting { target: food_entity });
                commands.entity(entity).insert(Pathing { path: vec![], destination: food_position.clone() });
                found_food = true;
                break;
            }
        }
        if !found_food {
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
        if (brain.task != Some(Task::Work)) { continue; }
        brain.task = Some(Task::Meander);
    }
}

pub fn task_system_meander(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain, &mut Position, &mut Transform), Without<TileType>>,
    tile_types: Query<(&Position, &TileType)>,
) {
    for (entity, mut brain, mut position, mut transform) in query.iter_mut() {
        // There is no work right now.
        if (brain.task != Some(Task::Meander)) { continue; }
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
