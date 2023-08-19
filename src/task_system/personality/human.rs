use crate::prelude::*;

pub fn human(
    entity: Entity,
    mut brain: Mut<Brain>,
    mut physical_body: Mut<PhysicalBody>,
    position: &Position,
    nest: Option<&Nest>,
    potential_targets: &Query<(Entity, Option<&Object>, Option<&Zone>, Option<&WorkTarget>, &Position)>,
    already_targeted: &Vec<Entity>,
) {
    if brain.task != Some(Task::Personality) { return; }
    if !brain.personality.contains(&PersonalityTrait::Human) { return; }
    let targets_to_choose_from: Vec<(Entity, u128)> = Vec::new();
    // Anything to DO?
    for (target_entity, target_object, target_zone, target_worktarget, target_position) in potential_targets.iter() {
        if already_targeted.contains(&target_entity) { continue; }
        if target_entity == entity { continue; }
        let distance = position.distance(&target_position);
        if distance > 100 { continue; }
        // TARGET OBJECTS
        if target_object.is_some() {
            let target_object = target_object.unwrap();
            if target_object.itemtype.is_choppable().0.is_some() && target_worktarget.is_some() {
                brain.task = Some(Task::Chop);
                return;
            }
            if target_object.itemtype.is_forageable().0.is_some() {
                brain.task = Some(Task::Forage);
                return;
            }
            if target_object.under_construction {
                brain.task = Some(Task::Construct);
                return;
            }
            // if target_object.itemtype.is_harvestable().0.is_some() {
            //     brain.task = Some(Task::Harvest);
            //     return;
            // }
        }
        // TARGET ZONES
        if target_zone.is_some() {
            let target_zone = target_zone.unwrap();
            if target_zone.zone_type == ZoneType::Farm {
                brain.task = Some(Task::Plant);
                return;
            }
            if target_zone.zone_type == ZoneType::Construction {
                brain.task = Some(Task::Construct);
                return;
            }
        }
        
    }
    brain.motivation = Some(Motivation::Meander);
    brain.task = None;
}