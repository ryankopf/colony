use super::prelude::*;

#[derive(Resource)]
pub struct TileHash {
    pub hash: std::collections::HashMap<Position, TileType>,
}