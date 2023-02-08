use std::thread::current;
use bevy::prelude::*;

use crate::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::First,
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(movement_along_path),
        )
        .add_system_set_to_stage(
            CoreStage::First,
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(movement_path_generating),
        );
    }
}

pub fn movement_toward_attackable(
    mut commands: Commands,
    attackers: Query<(Entity, &Position), (With<MoveTowardsNearestAttackable>, Without<Targeting>)>,
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
            commands.entity(attacker).insert( Pathing { path: vec![], destination: target_position.clone() });
        }
    }

}
#[derive(Clone, Debug)]
struct Node {
    position: Position,
    g: i32,
    h: i32,
    f: i32,
    parent: Option<Position>,
}
pub fn movement_path_generating(
    mut entities: Query<(&Position, &mut Pathing)>,
    //tiles: Query<(&Position, &TileType), With<MapTile>>,
    tilehash: Res<TileHash>,
) {
    let tiletypes: &std::collections::HashMap<Position, TileType> = &tilehash.hash;
    for (start_position, mut pathing) in entities.iter_mut() {
        let destination = pathing.destination;
        if pathing.path.len() != 0 { continue; }
        
        // F = G + H
        // G = distance from start
        // H = distance from end
        let mut openlist: std::collections::HashMap<Position, Node> = std::collections::HashMap::new();
        let mut closedlist: std::collections::HashMap<Position, Node> = std::collections::HashMap::new();
        openlist.insert(start_position.clone(), Node { position: start_position.clone(), g: 0, h: 0, f: 0, parent: None });
        while !openlist.is_empty() {
            let mut current_node = None;
            let mut lowest_f = -1;
            for (position, node) in openlist.iter() {
                if lowest_f == -1 || node.f < lowest_f {
                    current_node = Some(node);
                    lowest_f = node.f;
                }
            }
            if let None = current_node {
                break;
            }
            let current_node = current_node.unwrap().clone();
            let mut current_position = current_node.position.clone();
            openlist.remove(&current_position);
            // Add n to the CLOSED list
            let g = current_node.g + 1;
            let h = current_position.distance(&destination);
            let f = g + h;
            closedlist.insert(current_position.clone(), Node { position: current_position.clone(), g: g, h: h, f: f, parent: current_node.parent });
            // IF n is the same as the goal, we have a solution. Backtrack to find the path.
            if current_position == destination {
                let mut nodelist: Vec<Position> = vec![];
                loop {
                    nodelist.push(current_position.clone());
                    if let None = closedlist.get(&current_position) { break; }
                    match closedlist.get(&current_position).unwrap().parent {
                        None => break,
                        _ => current_position = closedlist.get(&current_position).unwrap().parent.clone().unwrap(),
                    }
                }
                pathing.path = nodelist;
                break;
            }
            let mut neighbors: Vec<Position> = vec![];
            neighbors.push(Position { x: current_position.x + 1, y: current_position.y, z: 0 });
            neighbors.push(Position { x: current_position.x - 1, y: current_position.y, z: 0 });
            neighbors.push(Position { x: current_position.x, y: current_position.y + 1, z: 0 });
            neighbors.push(Position { x: current_position.x, y: current_position.y - 1, z: 0 });

            for neighbor in neighbors {
                if tiletypes.get(&neighbor).is_none() { continue; }
                if tiletypes.get(&neighbor).unwrap().clone() == TileType::Wall {
                    continue;
                }
                let h = neighbor.distance(&destination);
                let g = current_node.g + 1;
                let f = g + h;
                if openlist.contains_key(&neighbor) {
                    if g > openlist.get(&neighbor).unwrap().g {
                        continue;
                    }
                }
                if closedlist.contains_key(&neighbor) {
                    if g > closedlist.get(&neighbor).unwrap().g {
                        continue;
                    }
                }
                openlist.remove(&neighbor);
                closedlist.remove(&neighbor);
                openlist.insert(neighbor.clone(), Node { position: neighbor.clone(), g: g, h: h, f: f, parent: Some(current_position.clone()) });

            }
        }
    }
}
// pub fn movement_path_generating_old(
//     mut commands: Commands,
//     mut entities: Query<(&Position, &mut Pathing)>,
//     tiles: Query<(Entity, &Position, &TileType), With<MapTile>>,
// ) {
//     for (start_position, mut pathing) in entities.iter_mut() {
//         let destination = pathing.destination;
//         if pathing.path.len() != 0 { continue; }
//         println!("Pathing: {:?} -> {:?}", start_position, pathing.destination);
//         let mut tiletypes: std::collections::HashMap<Position, TileType> = std::collections::HashMap::new();
//         for (tile_entity, tile_position, tile_type) in tiles.iter() {
//             tiletypes.insert(tile_position.clone(), tile_type.clone());
//         }
//         // F = G + H
//         // G = distance from start
//         // H = distance from end
//         //let mut path: Vec<Position> = vec![];
//         let mut openlist: Vec<Position> = vec![];
//         let mut gs: std::collections::HashMap<Position, i32> = std::collections::HashMap::new();
//         let mut h: std::collections::HashMap<Position, i32> = std::collections::HashMap::new();
//         let mut fs: std::collections::HashMap<Position, i32> = std::collections::HashMap::new();
//         let mut nodes: std::collections::HashMap<Position, Position> = std::collections::HashMap::new();
//         let mut closedlist: Vec<Position> = vec![];
//         openlist.push(start_position.clone());
//         while (!openlist.is_empty()) {
//             let mut current_position = openlist.pop().unwrap();
//             gs.insert(current_position.clone(), 0);
//             //h.insert(current_position.clone(), current_position.distance(&destination));
//             h.insert(current_position.clone(), 0);
//             fs.insert(current_position.clone(), 0);
//             if current_position == destination {
//                 pathing.path = closedlist; //
//                 pathing.path.reverse();
//                 break;
//             }
//             let mut neighbors: Vec<Position> = vec![];
//             neighbors.push(Position { x: current_position.x + 1, y: current_position.y, z: 0 });
//             neighbors.push(Position { x: current_position.x - 1, y: current_position.y, z: 0 });
//             neighbors.push(Position { x: current_position.x, y: current_position.y + 1, z: 0 });
//             neighbors.push(Position { x: current_position.x, y: current_position.y - 1, z: 0 });

//             for neighbor in neighbors {
//                 if tiletypes.get(&neighbor).unwrap().clone() == TileType::Wall {
//                     continue;
//                 }
//                 if closedlist.contains(&neighbor) {
//                     continue;
//                 }
//                 let g = gs.get(&current_position).unwrap().clone() + 1;
//                 gs.insert(neighbor.clone(), g);
//                 let h = neighbor.distance(&destination);
//                 let f = g + h;
//                 if openlist.contains(&neighbor) {
//                     if g > gs.get(&neighbor).unwrap().clone() {
//                         continue;
//                     }
//                 }

//                 openlist.push(neighbor.clone());
//             }
//             closedlist.push(current_position.clone());
//         }
        
//         println!("path: {:?}", pathing.path);
//     }
// }
pub fn movement_along_path(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Position, &mut Pathing, &mut Transform)>,
) {
    for (entity, mut position, mut pathing, mut transform) in entities.iter_mut() {
        if pathing.path.len() == 0 { continue; }
        let next_position = pathing.path.pop().unwrap();
        *position = next_position;
        let next_transform = next_position.to_transform();
        transform.translation.x = next_transform.translation.x as f32;
        transform.translation.y = next_transform.translation.y as f32;
        if pathing.path.len() == 0 {
            commands.entity(entity).remove::<Pathing>();
        }
    }
}
