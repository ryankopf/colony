use crate::prelude::*;

pub fn combat_system_ranged(
    _commands: Commands,
    mut query: Query<(&mut Brain, &mut PhysicalBody)>
) {
    for (mut brain, mut physical_body) in query.iter_mut() {
        if brain.task != Some(Task::Fight) { continue; }
        
    }
}