use crate::prelude::*;

pub fn vicious(
    entity: Entity,
    mut brain: Mut<Brain>,
    mut physical_body: Mut<PhysicalBody>,
    _position: &Position,
    nest: Option<&Nest>,
    potential_targets: &Vec<(Entity, Position)>
) {
    if brain.task != Some(Task::Personality) { return; }
    if !brain.personality.contains(&PersonalityTrait::Vicious) { return; }
    brain.motivation = Some(Motivation::Rage);
    brain.task = Some(Task::Fight);
}