use crate::prelude::*;

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
                commands.entity(entity).insert(Pathing { path: vec![], destination: bed_position.clone(), ..default() });
                found_bed = true;
                break;
            }
        }
        if !found_bed {
            brain.task = Some(Task::Sleeping);
        }
    }
}

pub fn task_system_sleeping(
    mut commands: Commands,
    mut query: Query<(&mut Brain, &mut Status)>
) {
    for (mut brain, mut status) in query.iter_mut() {
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