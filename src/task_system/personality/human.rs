use crate::prelude::*;

pub fn human(
    entity: Entity,
    mut brain: Mut<Brain>,
    mut physical_body: Mut<PhysicalBody>,
    _position: &Position,
    nest: Option<&Nest>,
    potential_targets: &Query<(Entity, &Object, &Position)>,
    already_targeted: &Vec<Entity>,
) {
    if brain.task != Some(Task::Personality) { return; }
    if !brain.personality.contains(&PersonalityTrait::Human) { return; }
    
    // Anything to defend against?
    for (target_entity, target_object, target_position) in potential_targets.iter() {
        // if entity == *target_entity { continue; }
        // if target_position.distance(&nest.position) < 10 {
        //     physical_body.danger = Some(Danger { danger_type: DangerType::Attacked, danger_source: Some(*target_entity) });
        //     brain.remotivate();
        //     break;
        // }
    }
    brain.motivation = Some(Motivation::Meander);
    brain.task = None;
}