use crate::prelude::*;

pub fn task_system_work(mut query: Query<(Entity, &mut Brain, &Position), Without<Targeting>>) {
    for (_entity, mut brain, _position) in query.iter_mut() {
        // There is no work right now.
        if brain.task != Some(Task::Work) {
            continue;
        }
        let mut rng = rand::thread_rng();
        // Generate number between 0 and 10
        let number: u8 = rng.gen_range(0..5);
        match number {
            0 => brain.task = Some(Task::Forage),
            1 => brain.task = Some(Task::Chop),
            2 => brain.task = Some(Task::Plant),
            // 3 => brain.task = Some(Task::Harvest),
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
