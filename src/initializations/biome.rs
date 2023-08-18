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
        plants: vec![ItemType::Cabbage, ItemType::Carrot, ItemType::PineTree, ItemType::PineTree, ItemType::PineTree, ItemType::PineTree, ItemType::PineTree,
        ItemType::CedarTree, ItemType::ThornBush, ItemType::Weed, ItemType::CactusRound],
        plant_scarcity: vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        plant_overall_scarcity: 10,
        tiles: vec![TileType::Grass, TileType::Grass, TileType::Grass, TileType::Grass, TileType::Grass, TileType::Grass, TileType::Grass,
        TileType::Grass, TileType::Grass, TileType::Grass, TileType::Grass, TileType::Dirt, TileType::Gravel],
        objects: vec![ItemType::StatuePillar3,ItemType::StatueCat,ItemType::StatueDragon, ItemType::Moss1, ItemType::Moss2, ItemType::LeafyDebris1],
        objects_scarcity: vec![1, 1, 1],
        objects_overall_scarcity: 200,
    }
}