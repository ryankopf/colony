use crate::prelude::*;

// Make Plugin
pub struct BiomePlugin;

impl Plugin for BiomePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(starting_biome())
        ;
    }
}

pub fn starting_biome() -> Biome {
    Biome {
        name: "Forest".to_string(),
        plants: vec![PlantType::Cabbage, PlantType::PineTree, PlantType::PineTree, PlantType::PineTree, PlantType::PineTree, PlantType::PineTree,
        PlantType::CedarTree, PlantType::ThornBush, PlantType::Weed, PlantType::CactusRound],
        tiles: vec![TileType::Grass, TileType::Grass, TileType::Grass, TileType::Grass, TileType::Grass, TileType::Grass, TileType::Grass,
        TileType::Grass, TileType::Grass, TileType::Grass, TileType::Grass, TileType::Dirt, TileType::Gravel],
    }
}