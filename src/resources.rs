use super::prelude::*;

#[derive(Resource)]
pub struct TileHash {
    pub hash: std::collections::HashMap<Position, TileType>,
}


#[derive(Resource)]
pub struct SpriteSheet(pub Handle<TextureAtlas>);

#[derive(Resource)]
pub struct Dragging {
    pub dragging: bool,
    pub start_position: Option<Position>,
}
