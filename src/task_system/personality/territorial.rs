use crate::prelude::*;

pub fn territorial(
    entity: Entity,
    mut brain: Mut<Brain>,
    mut physical_body: Mut<PhysicalBody>,
    position: &Position,
    nest: Option<&Nest>,
    potential_targets: &Vec<(Entity, Position)>
) {
    if brain.task != Some(Task::Personality) { return; }
    if !brain.personality.contains(&PersonalityTrait::Territorial) { return; }
    if nest.is_none() { return; }
    let nest = nest.unwrap();
    // Anything to defend against?
    for (target_entity, target_position) in potential_targets.iter() {
        if entity == *target_entity { continue; }
        if target_position.distance(&nest.position) < 10 {
            physical_body.danger = Some(Danger { danger_type: DangerType::Attacked, danger_source: Some(*target_entity) });
            brain.remotivate();
            break;
        }
    }
    brain.motivation = Some(Motivation::Meander);
    brain.task = None;
}