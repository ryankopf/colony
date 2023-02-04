use bevy::prelude::*;
use super::{Position, MapTile, SizeXYZ};
use super::constants::{COLOR_BLUE, COLOR_GRAY, COLOR_GREEN};
use super::prelude::*;

#[derive(Default)]
pub struct Map {
    pub tiles : Vec<TileType>,
    pub rooms : Vec<Rect>,
    pub width : i32,
    pub height : i32,
    pub revealed_tiles : Vec<bool>,
    pub visible_tiles : Vec<bool>
}

pub fn generate_map(mut commands: Commands) {
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_LENGTH {
            let tyle_type = if x == 0 || x == MAP_WIDTH - 1 || y == 0 || y == MAP_LENGTH - 1 {
                TileType::Wall
            } else {
                TileType::Floor
            };
            spawn_tile(&mut commands, Position { x, y, z: 0 }, tyle_type);
        }
    }
}


pub struct MapTiles(Vec<Entity>);

fn spawn_tile(commands: &mut Commands, position: Position, tile_type: TileType) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: match tile_type { TileType::Wall => COLOR_GRAY, _ => COLOR_GREEN },
                ..default()
            },
            ..default()
        })
        .insert(MapTile)
        .insert(position)
        .insert(tile_type)
        .insert(SizeXYZ::cube(1.0));
}