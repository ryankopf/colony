use super::prelude::*;

#[derive(Resource)]
pub struct TileHash {
    pub hash: std::collections::HashMap<Position, TileType>,
}


#[derive(Resource)]
pub struct SpriteSheet(pub Handle<TextureAtlas>);

#[derive(Resource)]
pub struct Biome {
    pub name: String,
    pub plants: Vec<ItemType>,
    pub plant_scarcity: Vec<u8>,
    pub plant_overall_scarcity: i32,
    pub tiles: Vec<TileType>,
    pub objects: Vec<ItemType>,
    pub objects_scarcity: Vec<u8>,
    pub objects_overall_scarcity: i32,
}

#[derive(Resource)]
pub struct Dragging {
    pub dragging: bool,
    pub start_position: Option<Position>,
    pub looking_for: SelectableType,
    pub zone_type: ZoneType,
    pub item_type: ItemType,
}

impl Default for Dragging {
    fn default() -> Self {
        Self {
            dragging: false,
            start_position: None,
            looking_for: SelectableType::Foragable,
            zone_type: ZoneType::Farm,
            item_type: ItemType::WallWood,
        }
    }
}

// Make Resource to hold font.
#[derive(Resource)]
pub struct MyFont(pub Handle<Font>);

#[derive(Resource, Default)]
pub struct SelectedObjectInformation {
    pub info: Vec<String>,
}
#[derive(Resource, Default)]
pub struct InfoPanelInformation {
    pub name: String,
    pub info: Vec<String>,
    pub needs: Vec<String>,
    pub attributes: Vec<String>,
    pub skills: Vec<String>,
}

#[derive(Resource)]
pub struct MenuState {
    pub state: MenuStates,
}