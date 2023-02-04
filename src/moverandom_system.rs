use bevy::prelude::*;
use rand::prelude::random;
use super::{Position, MapTile, SizeXYZ, MoveRandom};
use super::map::TileType;

pub fn movement_random(
    //segments: ResMut<SnakeSegments>,
    mut entities: Query<(Entity, &mut MoveRandom, &mut Position, Without<TileType>)>,
    //mut positions: Query<&mut Position>,
    mut tile_types: Query<(&Position, &mut TileType)>,
) {
    //let mut head_position = Position { x: 0, y: 0, z: 0 };
    for (entity, mut move_random, mut position, _) in entities.iter_mut() {
        let mut new_position = *position;
        let dir = random::<i32>() % 4;
        match dir {
            0 => new_position.y += 1,
            1 => new_position.y -= 1,
            2 => new_position.x -= 1,
            3 => new_position.x += 1,
            _ => {}
        }
        for (tile_position, mut tile_type) in tile_types.iter_mut() {
            let mut p2 = new_position;
            p2.z = 0;
            if *tile_position == p2 {
                if *tile_type != TileType::Wall {
                    *position = new_position;
                }
            }
        }
        //*position = new_position;
    }
}