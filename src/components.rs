use bevy::prelude::*;
use super::prelude::*;

#[derive(Component, Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl Position {
    pub fn to_transform(&self) -> Transform {
        Transform::from_xyz(
            self.x as f32 * TILE_SIZE,
            self.y as f32 * TILE_SIZE,
            self.z as f32,
        )
    }
    pub fn to_transform_layer(&self, layer: f32) -> Transform {
        Transform::from_xyz(
            self.x as f32 * TILE_SIZE,
            self.y as f32 * TILE_SIZE,
            self.z as f32 + layer,
        )
    }
    pub fn distance(&self, other: &Position) -> i32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32).sqrt() as i32
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
    Paused,
}

#[derive(Component, PartialEq, Clone, Debug)]
pub enum TileType {
    Wall, Floor
}

#[derive(Component)]
pub struct PauseOverlay;

#[derive(Component)]
pub struct Food {
    pub nutrition: f32,
    pub spoilage: f32,
    pub spoilage_rate: f32,
}
impl Default for Food {
    fn default() -> Self {
        Food {
            nutrition: 10.0,
            spoilage: 1.0,
            spoilage_rate: 0.1,
        }
    }
}

#[derive(Component)]
pub struct HasName {
    pub name: String,
}

#[derive(Component)]
pub struct IsName;

#[derive(Component)]
pub struct HasNameShown;

#[derive(Component)]
pub struct TextName;

#[derive(Component)]
pub struct HighlightBox;

#[derive(Component)]
pub struct Highlighted;

impl Default for Highlighted {
    fn default() -> Self {
        Highlighted {}
    }
}

#[derive(Component)]
pub struct Status {
    pub needs_food: Option<NeedsFood>,
    pub needs_entertainment: Option<NeedsEntertainment>,
    pub needs_sleep: Option<NeedsSleep>,
    pub index: usize,
    pub crisis: Option<String>,
    pub danger: Option<String>,
    pub injured: bool
}
#[derive(Component)]
pub struct Brain {
    pub motivation: Option<Motivation>,
    pub task: Option<Task>,
    pub order: Option<String>,
}
impl Brain {
    pub fn remotivate(&mut self) {
        self.motivation = None;
        self.task = None;
        self.order = None;
    }
}
impl Default for Brain {
    fn default() -> Self {
        Brain {
            motivation: None,
            task: None,
            order: None,
        }
    }
}

#[derive(Component)]
pub struct Foragable;

#[derive(Component)]
pub struct Choppable;

#[derive(Component)]
pub struct GiveMeAName;

#[derive(Component)]
pub struct Plant {
    pub growth: f32,
    pub plant_type: PlantType,
}

#[derive(Component)]
pub struct WorkMarker;

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum Task {
    Crisis, Flee, Fight, Eat, Hospital, Sleep, Sleeping, Play, Order, Work, Meander, Idle,
    Doctor, Forage, Harvest, Mine, Chop, Construct, Hunt, Milk, Cook, Fish, Craft, Clean, Haul // Forms of work
}
#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum Motivation {
    Crisis, Order, Danger, Hunger, Thirst, Tired, Bored, Injured, Sick, Happy, Sad, Angry, Lonely, Love, Fear, Hate, Work, Meander, Idle
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum PlantType {
    PineTree, OakTree, CedarTree, Bush, BerryBush
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum SelectableType {
    Foragable, Choppable, Mineable, Constructable, Harvestable, Unselecting
}

#[derive(Component)]
pub struct WorkTarget;

pub enum EdiblePlantTypes {
    BerryBush
}

#[derive(Component)]
pub struct Renderable {
    //pub glyph: rltk::FontCharType,
    pub fg: Color,
    pub bg: Color,
}

#[derive(Component)]
pub struct Bed;

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles : Vec<Position>,
    pub range : i32,
    pub dirty : bool
}

#[derive(Component)]
pub struct MapTile;

#[derive(Component)]
pub struct MoveRandom;

#[derive(Component)]
pub struct MonsterGenerator;

#[derive(Component)]
pub struct GeneratedBy {
    pub entity: Entity,
}
#[derive(Component)]
pub struct Targeting {
    pub target: Entity,
}

#[derive(Component)]
pub struct Pathing {
    pub path: Vec<Position>,
    pub destination: Position,
}

#[derive(Component)]
pub struct MoveTowardsTarget;

#[derive(Component)]
pub struct MoveTowardsNearestAttackable;

#[derive(Component)]
pub struct Attackable;

#[derive(Component)]
pub struct SizeXYZ {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}
impl SizeXYZ {
    pub fn cube(x: f32) -> Self {
        Self {
            width: x,
            height: x,
            depth: x,
        }
    }
    pub fn flat(x: f32) -> Self {
        Self {
            width: x,
            height: x,
            depth: 0.1,
        }
    }
    pub fn flat_2(x: f32) -> Self {
        Self {
            width: x,
            height: x,
            depth: 1.0,
        }
    }
}

// NEEDS

#[derive(Component)]
pub struct NeedsFood {
    pub current: f32,
    pub max: f32,
    pub rate: f32,
}
#[derive(Component)]
pub struct NeedsEntertainment {
    pub current: f32,
    pub max: f32,
    pub rate: f32,
}
#[derive(Component)]
pub struct NeedsSleep {
    pub current: f32,
    pub max: f32,
    pub rate: f32,
}

pub enum SPRITES {
    LOGS = 151,
}

#[derive(Component)]
pub struct Logs;

impl Default for Logs {
    fn default() -> Self {
        Logs {
        }
    }
}