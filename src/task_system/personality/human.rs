use crate::prelude::*;

pub fn human(
    entity: Entity,
    mut brain: Mut<Brain>,
    mut physical_body: Mut<PhysicalBody>,
    position: &Position,
    potential_targets: &Query<(Entity, Option<&Object>, Option<&Zone>, Option<&WorkTarget>, &Position)>,
    already_targeted: &Vec<Entity>,
    obstacles: &std::collections::HashSet<Position>,
    tiletypes: &std::collections::HashMap<Position, TileType>,
) {
    if brain.task != Some(Task::Personality) { return; }
    if !brain.personality.contains(&PersonalityTrait::Human) { return; }
    let targets_to_choose_from: Vec<(Entity, u128)> = Vec::new();
    let object_positions: Vec<Position> = potential_targets.iter()
        .filter(|(_, object, _, _, _)| object.is_some())
        .map(|(_, _, _, _, position)| *position)
        .collect();
    // Anything to DO?
    let mut i = 0;
    for (target_entity, target_object, target_zone, target_worktarget, target_position) in potential_targets.iter() {
        // i += 1;
        // println!("Target {} of {}", i, potential_targets.iter().count());
        
        if already_targeted.contains(&target_entity) { continue; }
        if target_entity == entity { continue; }
        let distance = position.distance(&target_position);
        if distance > 50 { continue; }
        let is_reachable = || crate::is_position_reachable(position, target_position, obstacles, tiletypes);
        
        // TARGET OBJECTS
        if target_object.is_some() {
            let target_object = target_object.unwrap();
            if target_worktarget.is_some() {
                if target_object.itemtype.is_choppable().0.is_some() {
                    if set_task(&mut brain, Task::Chop, is_reachable) { return; }
                }
                if target_object.itemtype.is_forageable().0.is_some() {
                    if set_task(&mut brain, Task::Forage, is_reachable) { return; }
                }
            }
            if target_object.under_construction {
                if set_task(&mut brain, Task::Construct, is_reachable) { return; }
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
                if ! object_positions.contains(&target_position) { // Ensure nothing is already planted there.
                    if set_task(&mut brain, Task::Plant, is_reachable) { return; }
                }
            }
            if target_zone.zone_type == ZoneType::Construction {
                if set_task(&mut brain, Task::Construct, is_reachable) { return; }
            }
        }
    }
    brain.motivation = Some(Motivation::Meander);
    brain.task = Some(Task::Meander);
}

fn set_task(
    brain: &mut Mut<Brain>,
    task: Task,
    is_reachable: impl FnOnce() -> bool,
) -> bool {
    if !is_reachable() { return false; }
    brain.task = Some(task);
    return true;
}