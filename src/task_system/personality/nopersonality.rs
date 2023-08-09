use crate::prelude::*;

pub fn nopersonality(
    entity: Entity,
    mut brain: Mut<Brain>,
    mut physical_body: Mut<PhysicalBody>,
    position: &Position,
    _: Option<&Nest>
) {
    if brain.task != Some(Task::Personality) { return; }
    if !brain.personality.is_empty() { return; } 
    
    brain.motivation = Some(Motivation::Meander);
    brain.task = None;
}