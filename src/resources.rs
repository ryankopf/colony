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
    pub looking_for: SelectableType,
}

impl Default for Dragging {
    fn default() -> Self {
        Self {
            dragging: false,
            start_position: None,
            looking_for: SelectableType::Foragable,
        }
    }
}

// Make Resource to hold font.
#[derive(Resource)]
pub struct MyFont(pub Handle<Font>);

#[derive(Resource)]
pub struct SelectedObjectInformation {
    // It's a vec of strings.
    pub info: Vec<String>,
}

impl Default for SelectedObjectInformation {
    fn default() -> Self {
        Self { info: vec![] }
    }
}