use bevy::prelude::*;
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
    let mut tiletypes: std::collections::HashMap<Position, TileType> = std::collections::HashMap::new();
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_LENGTH {
            let tyle_type = if x == 0 || x == MAP_WIDTH - 1 || y == 0 || y == MAP_LENGTH - 1 {
                TileType::Wall
            } else {
                TileType::Floor
            };
            spawn_tile(&mut commands, Position { x, y, z: 0 }, tyle_type.clone());
            tiletypes.insert( Position { x, y, z: 0 }, tyle_type);
        }
    }
    commands.insert_resource(TileHash { hash: tiletypes });
}
pub fn _update_map_tiles(
    mut commands: Commands,
    tiles: Query<(&Position, &TileType), With<MapTile>>,
) {
    let mut tiletypes: std::collections::HashMap<Position, TileType> = std::collections::HashMap::new();
    for (tile_position, tile_type) in tiles.iter() {
        tiletypes.insert(tile_position.clone(), tile_type.clone());
    }
    println!("tiletypes: {:?}", tiletypes);
    commands.insert_resource(TileHash { hash: tiletypes });
}

fn spawn_tile(commands: &mut Commands, position: Position, tile_type: TileType) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: match tile_type { TileType::Wall => Color::GRAY, _ => Color::GREEN },
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            ..default()
        })
        .insert(MapTile)
        .insert(position)
        .insert(tile_type)
        .insert(SizeXYZ::flat(TILE_SIZE))
        .insert(position.to_transform())
        ;
}