use crate::prelude::*;

pub fn combat_system_melee(
    _commands: Commands,
    mut query: Query<(&mut Brain, &mut PhysicalBody)>
) {
    for (mut brain, mut physical_body) in query.iter_mut() {
        if brain.task != Some(Task::Fight) { continue; }
        
        // if let Some(n) = &mut physical_body.needs_entertainment {
        //     n.current += 10.0;
        //     if n.current >= n.max {
        //         brain.motivation = None;
        //         brain.task = None;
        //     }
        // }
    }
}