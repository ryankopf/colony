use crate::prelude::*;

pub fn personality_nopersonality(
    mut entities: Query<(Entity, &mut Brain, &mut PhysicalBody, &Position)>
) {
    for (entity, mut brain, mut physical_body, position) in entities.iter_mut() {
        if brain.task != Some(Task::Personality) { continue; }
        if !brain.personality.is_empty() { continue; } 
        
        brain.motivation = Some(Motivation::Meander);
        brain.task = None;
    }
}