use bevy::prelude::*;

use crate::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems( Update, (movement_path_generating, clear_unreachable_paths))
        .add_systems(
            Update,
            movement_along_path
            .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
            .run_if(in_state(GameState::InGame))
        )
        .add_systems(
            PostUpdate,
            update_paths_for_moving_targets
        )
        ;
    }
}

pub fn movement_toward_attackable(
    mut commands: Commands,
    attackers: Query<(Entity, &Position), (With<MoveTowardsNearestAttackable>, Without<Pathing>)>,
    attackables: Query<(Entity, &Position), With<Attackable>>
) {
    for (attacker, attacker_position) in attackers.iter() {
        let mut closest_distance = 9999;
        let mut closest_target = None;
        for (attackable, attackable_position) in attackables.iter() {
            let distance = attacker_position.distance(attackable_position);
            if distance < closest_distance {
                closest_distance = distance;
                closest_target = Some(attackable);
            }
        }
        if let Some(closest_target) = closest_target {
            commands.entity(attacker).insert(Targeting { target: closest_target });
            let target_position = attackables.get(closest_target).unwrap().1;
            commands.entity(attacker).insert( Pathing { path: vec![], destination: *target_position, ..default() });
        }
    }
}
#[derive(Clone, Debug)]
struct Node {
    position: Position,
    g: i32,
    f: i32,
    parent: Option<Position>,
}
pub fn movement_path_generating(
    mut entities: Query<(&Position, &mut Pathing)>,
    tilehash: Res<TileHash>,
) {
    let tiletypes: &std::collections::HashMap<Position, TileType> = &tilehash.hash;
    for (start_position, mut pathing) in entities.iter_mut() {
        // println!("Pathing: {:?} to destination: {:?} - Unreachable? {:?}", pathing.path, pathing.destination, pathing.unreachable);
        let destination = pathing.destination;
        if !pathing.path.is_empty() { continue; }
        pathing.path = generate_path(start_position, &destination, tiletypes);
        
        // F = G + H
        // G = distance from start
        // H = distance from end

        // let mut openlist: std::collections::HashMap<Position, Node> = std::collections::HashMap::new();
        // let mut closedlist: std::collections::HashMap<Position, Node> = std::collections::HashMap::new();
        // openlist.insert(*start_position, Node { position: *start_position, g: 0, f: 0, parent: None });
        // while !openlist.is_empty() {
        //     let mut current_node = None;
        //     let mut lowest_f = -1;
        //     for (_position, node) in openlist.iter() {
        //         if lowest_f == -1 || node.f < lowest_f {
        //             current_node = Some(node);
        //             lowest_f = node.f;
        //         }
        //     }
        //     if current_node.is_none() {
        //         break;
        //     }
        //     let current_node = current_node.unwrap().clone();
        //     let mut current_position = current_node.position;
        //     openlist.remove(&current_position);
        //     // Add n to the CLOSED list
        //     let g = current_node.g + 1;
        //     let h = current_position.distance(&destination);
        //     let f = g + h;
        //     closedlist.insert(current_position, Node { position: current_position, g, f, parent: current_node.parent });
        //     // IF n is the same as the goal, we have a solution. Backtrack to find the path.
        //     if current_position == destination {
        //         let mut nodelist: Vec<Position> = vec![];
        //         loop {
        //             nodelist.push(current_position);
        //             if closedlist.get(&current_position).is_none() { break; }
        //             match closedlist.get(&current_position).unwrap().parent {
        //                 None => break,
        //                 _ => current_position = closedlist.get(&current_position).unwrap().parent.unwrap(),
        //             }
        //         }
        //         pathing.path = nodelist;
        //         break;
        //     }
        //     let mut neighbors: Vec<Position> = vec![];
        //     neighbors.push(Position { x: current_position.x + 1, y: current_position.y, z: 0 });
        //     neighbors.push(Position { x: current_position.x - 1, y: current_position.y, z: 0 });
        //     neighbors.push(Position { x: current_position.x, y: current_position.y + 1, z: 0 });
        //     neighbors.push(Position { x: current_position.x, y: current_position.y - 1, z: 0 });

        //     for neighbor in neighbors {
        //         if tiletypes.get(&neighbor).is_none() { continue; }
        //         if tiletypes.get(&neighbor).unwrap().is_wall() {
        //             continue;
        //         }
        //         let h = neighbor.distance(&destination);
        //         let g = current_node.g + 1;
        //         let f = g + h;
        //         if openlist.contains_key(&neighbor) && g > openlist.get(&neighbor).unwrap().g {
        //             continue;
        //         }
        //         if closedlist.contains_key(&neighbor) && g > closedlist.get(&neighbor).unwrap().g {
        //             continue;
        //         }
        //         openlist.remove(&neighbor);
        //         closedlist.remove(&neighbor);
        //         openlist.insert(neighbor, Node { position: neighbor, g, f, parent: Some(current_position) });
        //     }
        // }
        // Unreachable!
        if pathing.path.is_empty() {
            pathing.unreachable = true;
        }
    }
}
fn generate_path(
    start_position: &Position,
    destination: &Position,
    tiletypes: &std::collections::HashMap<Position, TileType>,
) -> Vec<Position> {
    // F = G + H
    // G = distance from start
    // H = distance from end
    let mut openlist: std::collections::HashMap<Position, Node> = std::collections::HashMap::new();
    let mut closedlist: std::collections::HashMap<Position, Node> = std::collections::HashMap::new();
    let mut nodelist: Vec<Position> = vec![];
    openlist.insert(*start_position, Node { position: *start_position, g: 0, f: 0, parent: None });
    while !openlist.is_empty() {
        let mut current_node = None;
        let mut lowest_f = -1;
        for (_position, node) in openlist.iter() {
            if lowest_f == -1 || node.f < lowest_f {
                current_node = Some(node);
                lowest_f = node.f;
            }
        }
        if current_node.is_none() {
            break;
        }
        let current_node = current_node.unwrap().clone();
        let mut current_position = current_node.position;
        openlist.remove(&current_position);
        // Add n to the CLOSED list
        let g = current_node.g + 1;
        let h = current_position.distance(&destination);
        let f = g + h;
        closedlist.insert(current_position, Node { position: current_position, g, f, parent: current_node.parent });
        // IF n is the same as the goal, we have a solution. Backtrack to find the path.
        if current_position == *destination {
            loop {
                if current_position != *start_position { nodelist.push(current_position); }
                if closedlist.get(&current_position).is_none() { break; }
                match closedlist.get(&current_position).unwrap().parent {
                    None => break,
                    _ => current_position = closedlist.get(&current_position).unwrap().parent.unwrap(),
                }
            }
            return nodelist;
        }
        let mut neighbors: Vec<Position> = vec![];
        neighbors.push(Position { x: current_position.x + 1, y: current_position.y, z: 0 });
        neighbors.push(Position { x: current_position.x - 1, y: current_position.y, z: 0 });
        neighbors.push(Position { x: current_position.x, y: current_position.y + 1, z: 0 });
        neighbors.push(Position { x: current_position.x, y: current_position.y - 1, z: 0 });

        for neighbor in neighbors {
            if tiletypes.get(&neighbor).is_none() { continue; }
            if tiletypes.get(&neighbor).unwrap().is_wall() {
                continue;
            }
            let h = neighbor.distance(&destination);
            let g = current_node.g + 1;
            let f = g + h;
            if openlist.contains_key(&neighbor) && g > openlist.get(&neighbor).unwrap().g {
                continue;
            }
            if closedlist.contains_key(&neighbor) && g > closedlist.get(&neighbor).unwrap().g {
                continue;
            }
            openlist.remove(&neighbor);
            closedlist.remove(&neighbor);
            openlist.insert(neighbor, Node { position: neighbor, g, f, parent: Some(current_position) });

        }
    }

    nodelist
}


pub fn clear_unreachable_paths(
    mut commands: Commands,
    entities: Query<(Entity, &Pathing)>,
) {
    for (entity, pathing) in entities.iter() {
        if pathing.unreachable {
            commands.entity(entity).remove::<Pathing>();
        }
    }
}
pub fn update_paths_for_moving_targets(
    mut entities: Query<(&Position, &Targeting, &mut Pathing)>,
    targets: Query<(Entity, &Position)>,
    tilehash: Res<TileHash>,
) {
    let tiletypes: &std::collections::HashMap<Position, TileType> = &tilehash.hash;
    for (start_position, target, mut pathing) in entities.iter_mut() {
        if !pathing.moving_target { continue; }
        pathing.moving_target = false;
        for (target_entity, target_position) in targets.iter() {
            if target_entity == target.target {
                pathing.path = generate_path(start_position, &target_position, tiletypes);
                // println!("Path: {:?}", pathing.path)
            }
        }
    }
}

pub fn movement_along_path(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Position, &mut Pathing, &mut Transform)>,
) {
    for (entity, mut position, mut pathing, mut transform) in entities.iter_mut() {
        if pathing.path.is_empty() { continue; }
        let next_position = pathing.path.pop().unwrap();
        *position = next_position;
        let next_transform = next_position.to_transform();
        transform.translation.x = next_transform.translation.x;
        transform.translation.y = next_transform.translation.y;
        if pathing.path.is_empty() {
            commands.entity(entity).remove::<Pathing>();
        }
    }
}
