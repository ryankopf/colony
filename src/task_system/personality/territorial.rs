use crate::prelude::*;

pub fn personality_territorial(
    mut entities: Query<(Entity, &mut Brain, &mut PhysicalBody, &Position, Option<&Nest>)>
) {
    let potential_targets = entities.iter()
        .map(|(entity, _, _, position, _)| (entity, *position)) // Clone the Position data
        .collect::<Vec<(Entity, Position)>>();
    for (entity, mut brain, mut physical_body, position, nest) in entities.iter_mut() {
        if brain.task != Some(Task::Personality) { continue; }
        if !brain.personality.contains(&PersonalityTrait::Territorial) { continue; } 
        // Anything to defend against?
        for (target_entity, target_position) in potential_targets.iter() {
            if entity == *target_entity { continue; }
            if target_position.distance(position) < 5 {
                physical_body.danger = Some(Danger { danger_type: DangerType::Attacked, danger_source: Some(*target_entity) });
                brain.remotivate();
                break;
            }
        }
        brain.motivation = Some(Motivation::Meander);
        brain.task = None;
    }
}