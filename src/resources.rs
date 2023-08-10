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
    pub plants: Vec<PlantType>,
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
    pub plant_type: PlantType,
}

impl Default for Dragging {
    fn default() -> Self {
        Self {
            dragging: false,
            start_position: None,
            looking_for: SelectableType::Foragable,
            zone_type: ZoneType::Farm,
            plant_type: PlantType::Cabbage,
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