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

#[derive(States, Clone, Copy, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    InGame,
    MainMenu,
    Paused,
}

#[derive(PartialEq)]
pub enum MenuStates { // Sorted in order of display.
    Home, Tasks, Farm, Zone, Build, Craft
}

impl MenuStates {
    pub fn to_index(&self) -> usize {
        match self { // Sorted in order of display.
            MenuStates::Home => 0,
            MenuStates::Tasks => 1,
            MenuStates::Farm => 2,
            MenuStates::Zone => 3,
            MenuStates::Build => 4,
            MenuStates::Craft => 5,
        }
    }
}
#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum StrikeType {
    Hit
}
impl StrikeType {
    pub fn sprite_row_and_col(&self) -> (usize, usize) {
        match self {
            StrikeType::Hit => (26, 9),
        }
    }
    pub fn sprite_index(&self) -> usize {
        let (row, col) = self.sprite_row_and_col();
        row * 64 + col
    }
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum ActorType { // Entity? Character? Creature? Actor? Avatar? Unit? Agent?
    Dwarf,
    ManCrazy,
    Elf,
    Teen,
    Ranger,
    Woman,
    Man,
    Man2,
    ManCave,
    Pig,
    Rat,
    OrbSpider,
    Bear,
    Ant,
    Locust,
    Wasp,
    Dingo,
    Kangaroo,
    IceFox,
    BrownRat,
    Spider,
    Crab
}
impl ActorType {
    pub fn sprite_row_and_col(&self) -> (usize, usize) {
        match self {
            ActorType::Dwarf => (59 ,13),
            ActorType::ManCrazy => (59, 15),
            ActorType::Elf => (59, 18),
            ActorType::Teen => (60, 11),
            ActorType::Ranger => (59, 22),
            ActorType::Woman => (60, 48),
            ActorType::Man => (66, 46),
            ActorType::Man2 => (60, 21),
            ActorType::ManCave => (60, 25),
            ActorType::Pig => (64, 0),
            ActorType::Rat => (64, 19),
            ActorType::OrbSpider => (64, 20),
            ActorType::Bear => (64, 21),
            ActorType::Ant => (64, 22),
            ActorType::Locust => (64, 23),
            ActorType::Wasp => (64, 24),
            ActorType::Dingo => (64, 25),
            ActorType::Kangaroo => (64, 26),
            ActorType::IceFox => (64, 27),
            ActorType::BrownRat => (64, 28),
            ActorType::Spider => (64, 29),
            ActorType::Crab => (63, 29),
        }
    }
    pub fn sprite_index(&self) -> usize {
        let (row, col) = self.sprite_row_and_col();
        row * 64 + col
    }
}

#[derive(Component, PartialEq, Clone, Debug)]
pub enum TileType {
    Grass,
    Cave,
    Dirt,
    Gravel,
    Sand,
    Stone,
    Water,
    WallBrick,
    WallGame,
    WallMetal,
    WallStone,
    WallWood,
}

impl TileType {
    pub fn sprite_row_and_col(&self) -> (usize, usize) {
        match self {
            TileType::Grass => (9, 11),
            TileType::Cave => (13, 7),
            TileType::Dirt => (4, 1),
            TileType::Gravel => (7, 42),
            TileType::Sand => (7, 42),
            TileType::Stone => (3, 61),
            TileType::Water => (5, 12),
            TileType::WallGame => (7, 20),
            TileType::WallStone => (7, 21),
            TileType::WallWood => (7, 22),
            TileType::WallBrick => (4, 10),
            TileType::WallMetal => (7, 24),
        }
    }
    pub fn sprite_index(&self) -> usize {
        let (row, col) = self.sprite_row_and_col();
        row * 64 + col
    }
    pub fn is_wall(&self) -> bool {
        matches!(self, TileType::WallGame | TileType::WallStone | TileType::WallWood | TileType::WallBrick | TileType::WallMetal)

    }
}

#[derive(Component)]
pub struct PauseOverlay;

#[derive(Component)]
pub struct MainMenuOverlay;

pub trait HoverNote {
    fn hover_note(&self) -> String;
}

#[derive(Component)]
pub struct Food {
    pub nutrition: f32,
    pub spoilage: f32,
    pub spoilage_rate: f32,
    pub name: String,
}
impl Default for Food {
    fn default() -> Self {
        Food {
            nutrition: 10.0,
            spoilage: 1.0,
            spoilage_rate: 0.03,
            name: "Food".to_string(),
        }
    }
}
impl HoverNote for Food {
    fn hover_note(&self) -> String {
        let spoilage_percent = self.spoilage * 100.0;
        format!("Spoilage: {:.2}%", spoilage_percent)
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
#[derive(Default)]
pub struct Highlighted;

#[derive(Component)]
pub struct ClickedOn;

#[derive(Component)]
pub struct InGameButton;

// #[derive(Component)]
// pub struct CombatTarget {
//     pub target: Entity,
// }

#[derive(Component, Clone, Copy)]
pub struct Skill {
    pub experience: i32,
    pub exp_lost: i32, // Forgetting/atrophied skills. Easier to regain.
}
impl Skill {
    pub fn level(&self) -> i32 {
        if self.experience < 10000 {
            //0, 100, 400, 900, 1600, 2500, 3600, 6400, 8100, 10000
            (self.experience as f32 / 100.0).sqrt() as i32
        } else {
            // 12000, 14000, 18000, 26000, 42000, 74000, 138000, 266000, 513000, 10250000
            10 + ((self.experience-10000) as f32 / 1000.0).log2() as i32
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Attributeset {
    pub health: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}
impl Default for Attributeset {
    fn default() -> Self {
        Attributeset {
            health: 100,
            strength: 1,
            dexterity: 1,
            constitution: 1,
            intelligence: 1,
            wisdom: 1,
            charisma: 1,
        }
    }
}

#[derive(Clone, Copy, Component)]
pub enum AfflictionType {
    Bleeding,
    BrokenBone,
    Bruised,
    Burned,
    Concussion,
    Cut,
    Dehydration,
    Disease,
    Exhaustion,
    FoodPoisoning,
    Frostbite,
    Hypothermia,
    Infection,
    Inflammation,
    InternalBleeding,
    InternalInfection,
    InternalInflammation,
    InternalPain,
    InternalPoisoning,
    InternalSwelling,
    InternalTrauma,
    Pain,
    Poisoned,
    Swelling,
    Trauma,
    Wound,
    WoundInfection,
    WoundInflammation,
    WoundPain,
    WoundSwelling,
    WoundTrauma,
}
#[derive(Clone, Copy, Component)]
pub enum AfflictionLocation {
    Head,
    Torso,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
    LeftHand,
    RightHand,
    LeftFoot,
    RightFoot,
    LeftEye,
    RightEye,
    LeftEar,
    RightEar,
    LeftLung,
    RightLung,
    Heart,
    Stomach,
    Liver,
    Spleen,
    Kidneys,
    Bladder,
    Intestines,
    Genitals,
    Tail,
    Horns,
    Wings,
    Tentacles,
    Pseudopods,
    Claws,
    Teeth,
    Beak,
    Tongue,
    Trunk,
}

#[derive(Component, Clone, Copy)]
pub struct Affliction {
    pub affliction_type: AfflictionType,
    pub affliction_location: AfflictionLocation,
    pub duration: i32,
    pub severity: i32,
    pub worsening: bool,
}

#[derive(Component, Clone, Copy)]
pub struct Skillset {
    pub animal_raising: Skill,
    pub brawling: Skill,
    pub construction: Skill,
    pub cooking: Skill,
    pub crafting: Skill,
    pub doctoring: Skill,
    pub farming: Skill,
    pub fishing: Skill,
    pub foraging: Skill,
    pub hunting: Skill,
    pub mining: Skill,
    pub social: Skill,
    pub woodcutting: Skill,
}
impl Default for Skillset {
    fn default() -> Self {
        Skillset {
            animal_raising: Skill { experience: 0, exp_lost: 0 },
            brawling: Skill { experience: 0, exp_lost: 0 },
            construction: Skill { experience: 0, exp_lost: 0 },
            cooking: Skill { experience: 0, exp_lost: 0 },
            crafting: Skill { experience: 0, exp_lost: 0 },
            doctoring: Skill { experience: 0, exp_lost: 0 },
            farming: Skill { experience: 0, exp_lost: 0 },
            fishing: Skill { experience: 0, exp_lost: 0 },
            foraging: Skill { experience: 0, exp_lost: 0 },
            hunting: Skill { experience: 0, exp_lost: 0 },
            mining: Skill { experience: 0, exp_lost: 0 },
            social: Skill { experience: 0, exp_lost: 0 },
            woodcutting: Skill { experience: 0, exp_lost: 0 },
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum DangerType {
    Attacked,
    Fire,
    Freezing,
    Overheating,
}
#[derive(Component, Clone)]
pub struct Danger {
    pub danger_type: DangerType,
    pub danger_source: Option<Entity>,
}

pub trait InfoPanel {
    fn info_panel(&self) -> Vec<String>;
}

#[derive(Component, Clone)]
pub struct PhysicalBody {
    pub needs_food: Option<Need>,
    pub needs_entertainment: Option<Need>,
    pub needs_sleep: Option<Need>,
    pub index: usize,
    pub crisis: Option<String>,
    pub danger: Option<Danger>,
    pub injured: bool,
    pub afflictions: Vec<Affliction>,
    pub skillset: Skillset,
    pub attributes: Attributeset,
}
impl PhysicalBody {
    pub fn info_panel_needs(&self) -> Vec<String> {
        let mut info_lines = Vec::new();
        if let Some(needs_food) = &self.needs_food {
            info_lines.push(format!("Food: {:.2}%", needs_food.current / needs_food.max * 100.0));
        }
        if let Some(needs_entertainment) = &self.needs_entertainment {
            info_lines.push(format!("Entertainment: {:.2}%", needs_entertainment.current / needs_entertainment.max * 100.0));
        }
        if let Some(needs_sleep) = &self.needs_sleep {
            info_lines.push(format!("Sleep: {:.2}%", needs_sleep.current / needs_sleep.max * 100.0));
        }
        info_lines
    }
    pub fn info_panel_attributes(&self) -> Vec<String> {
        let mut info_lines = Vec::new();
        info_lines.push(format!("Health: {}", self.attributes.health));
        info_lines.push(format!("Strength: {}", self.attributes.strength));
        info_lines.push(format!("Dexterity: {}", self.attributes.dexterity));
        info_lines.push(format!("Constitution: {}", self.attributes.constitution));
        info_lines.push(format!("Intelligence: {}", self.attributes.intelligence));
        info_lines.push(format!("Wisdom: {}", self.attributes.wisdom));
        info_lines.push(format!("Charisma: {}", self.attributes.charisma));
        info_lines
    }
    pub fn info_panel_skills(&self) -> Vec<String> {
        let mut info_lines = Vec::new();
        info_lines.push(format!("Animal Raising: {} ({} xp)", self.skillset.animal_raising.level(), self.skillset.animal_raising.experience));
        info_lines.push(format!("Brawling: {} ({} xp)", self.skillset.brawling.level(), self.skillset.brawling.experience));
        info_lines.push(format!("Construction: {} ({} xp)", self.skillset.construction.level(), self.skillset.construction.experience));
        info_lines.push(format!("Cooking: {} ({} xp)", self.skillset.cooking.level(), self.skillset.cooking.experience));
        info_lines.push(format!("Crafting: {} ({} xp)", self.skillset.crafting.level(), self.skillset.crafting.experience));
        info_lines.push(format!("Doctoring: {} ({} xp)", self.skillset.doctoring.level(), self.skillset.doctoring.experience));
        info_lines.push(format!("Farming: {} ({} xp)", self.skillset.farming.level(), self.skillset.farming.experience));
        info_lines.push(format!("Fishing: {} ({} xp)", self.skillset.fishing.level(), self.skillset.fishing.experience));
        info_lines.push(format!("Foraging: {} ({} xp)", self.skillset.foraging.level(), self.skillset.foraging.experience));
        info_lines.push(format!("Hunting: {} ({} xp)", self.skillset.hunting.level(), self.skillset.hunting.experience));
        info_lines.push(format!("Mining: {} ({} xp)", self.skillset.mining.level(), self.skillset.mining.experience));
        info_lines.push(format!("Social: {} ({} xp)", self.skillset.social.level(), self.skillset.social.experience));
        info_lines.push(format!("Woodcutting: {} ({} xp)", self.skillset.woodcutting.level(), self.skillset.woodcutting.experience));
        info_lines
    }
}
#[derive(Component)]
pub struct Attacked {
    pub attacker: Entity,
}
#[derive(Component)]
pub struct Dying;

#[derive(PartialEq)]
pub enum Order {
    Eat,
    Hospital,
    Follow,
    Stay,
    Guard,
    Patrol,
    Wander,
    Work,
    Sleep,
    Drink,
    Play,
    Party,
    Socialize,
    Fight,
    Flee,
    Doctor,
    Forage,
    Plant,
    Harvest,
    Mine,
    Chop,
    Construct,
    Hunt,
    Milk,
    Cook,
    Fish,
    Craft,
    Clean,
    Haul,
    None,
}

#[derive(Component, Default)]
pub struct Brain {
    pub motivation: Option<Motivation>,
    pub task: Option<Task>,
    pub order: Option<Order>,
    pub personality: Vec<PersonalityTrait>,
    pub last_considered_personality_trait: Option<PersonalityTrait>,
}
impl Brain {
    pub fn remotivate(&mut self) {
        self.motivation = None;
        self.task = None;
        self.order = None;
    }
    pub fn add_personality_trait(&mut self, trait_: PersonalityTrait) {
        self.personality.push(trait_);
    }
    pub fn get_next_personality_trait(&mut self) -> Option<PersonalityTrait> {
        if self.last_considered_personality_trait.is_none() {
            // return the first personality
            if self.personality.len() > 0 {
                self.last_considered_personality_trait = Some(self.personality[0]);
                return Some(self.personality[0]);
            } else {
                return None;
            }
        }
        for (i, personality_trait) in self.personality.iter().enumerate() {
            if self.last_considered_personality_trait.unwrap() == *personality_trait {
                if i == self.personality.len() - 1 {
                    self.last_considered_personality_trait = None;
                    return None;
                } else {
                    self.last_considered_personality_trait = Some(self.personality[i+1]);
                    return Some(self.personality[i+1]);
                }
            }
        }
        return None;
    }
}
impl InfoPanel for Brain {
    fn info_panel(&self) -> Vec<String> {
        let mut info_lines = Vec::new();
        if let Some(motivation) = &self.motivation {
            info_lines.push(format!("Motivation: {:?}", motivation));
        }
        if let Some(task) = &self.task {
            info_lines.push(format!("Task: {:?}", task));
        }
        info_lines
    }
}

#[derive(Component, PartialEq, Copy, Clone)]
pub enum PersonalityTrait {
    // Traits for People
    Adventurous, Ambitious, Analytical, Airheaded, Artistic, Brave, Calm, Charismatic, Confident, Cowardly,
    Creative, Curious, Charitable, Cynical, Dumb, Eccentric, Energetic, Empath, Empathetic, Enthusiastic,
    Fearless, Friendly, Greedy, Impulsive, Jinxed, Loyal, Logical, Lucky, Mean, Mischievous,
    Nice, Optimistic, Patient, Pessimistic, Rebellious, Reliable, Sensitive, Shy, Smart, Stupid,
    Technophile, Timid, Tolerant, Trusting, Violent, Weak, Workaholic, Witty, Outgoing,
    // Traits for Creatures
    Creature, Social, Vicious, Territorial, Docile, 
}

#[derive(Component)]
pub struct Nest {
    pub position: Position,
}

#[derive(Component)]
pub struct Foragable;

#[derive(Component)]
pub struct Choppable;

#[derive(Component)]
pub struct SetNest;

#[derive(Component)]
pub struct GiveMeAName;

#[derive(Component)]
pub struct Plant {
    pub growth: f32,
    pub plant_type: PlantType,
}
impl HoverNote for Plant {
    fn hover_note(&self) -> String {
        format!("{:?} Growth: {:.2}%", self.plant_type, self.growth * 100.0)
    }
}

#[derive(Component)]
pub struct WorkMarker;

#[derive(Component)]
pub struct ZoneMarker;


#[derive(Component)]
pub struct Zone {
    pub zone_type: ZoneType,
    pub plant_type: PlantType,
    pub material_delivered: bool,
}

impl Default for Zone {
    fn default() -> Self {
        Zone {
            zone_type: ZoneType::Farm,
            plant_type: PlantType::Cabbage,
            material_delivered: false,
        }
    }
}

pub struct NearestEntity {
    pub entity: Entity,
    pub position: Position,
    pub distance: i32,
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum ZoneType {
    Farm, Pasture, Storage, Fishing, Hospital, Party, Meeting
}


#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum Task { // Sorted in order of prioritization.
    Crisis, Flee, Fight, Eat, Hospital, Sleep, Sleeping, Play, Order, Work, Personality, Meander, Idle,
    Doctor, Forage, Plant, Harvest, Mine, Chop, Construct, Hunt, Milk, Cook, Fish, Craft, Clean, Haul // Forms of work
}
#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum Motivation { // Sorted in order of prioritization.
    Crisis, Rage, Order, Danger, Hunger, Thirst, Tired, Injured, Sick, Bored, Happy, Sad, Angry, Lonely, Love, Fear, Hate, Work, Personality, Meander, Idle
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum ItemType {
    Cabbage,
    Carrot,
    CedarLog,
    PineLog,
    OakLog,
}

impl ItemType {
    pub fn sprite_row_and_col(&self) -> (usize, usize) {
        match self {
            ItemType::Cabbage => (94, 33),
            ItemType::Carrot => (94, 24),
            ItemType::CedarLog => (94, 30),
            ItemType::PineLog => (94, 30),
            ItemType::OakLog => (94, 30),
        }
    }
    pub fn sprite_index(&self) -> usize {
        let (row, col) = self.sprite_row_and_col();
        row * 64 + col
    }
    pub fn nutrition(&self) -> f32 {
        match self {
            ItemType::Cabbage => 10.0,
            ItemType::Carrot => 10.0,
            _ => 0.0,
        }
    }
    // pub fn spoilage(&self) -> f32 {
    //     match self {
    //         ItemType::Cabbage => 1.0,
    //         ItemType::Carrot => 1.0,
    //         _ => 0.0,
    //     }
    // }
    pub fn spoilage_rate(&self) -> f32 {
        match self {
            ItemType::Cabbage => 0.1,
            ItemType::Carrot => 0.1,
            _ => 0.01,
        }
    }
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum PlantType {
    Aloe,
    Azalea,
    Bush,
    Cabbage,
    CactusRound,
    CactusUp,
    Carrot,
    CedarTree,
    FlowerBush,
    PineTree,
    OakTree,
    ThornBush,
    Vine,
    Weed,
}

impl PlantType {
    pub fn is_edible(&self) -> bool {
        matches!(self, PlantType::Cabbage)
    }
    pub fn sprite_row_and_col(&self) -> (usize, usize) {
        match self {
            PlantType::Aloe => (67, 57),
            PlantType::Azalea => (67, 57),
            PlantType::Bush => (67, 57),
            PlantType::Cabbage => (94, 32),
            PlantType::CactusRound => (67, 57),
            PlantType::CactusUp => (67, 57),
            PlantType::Carrot => (94, 31),
            PlantType::CedarTree => (13, 15),
            PlantType::PineTree => (13, 13),
            PlantType::OakTree => (13, 14),
            PlantType::ThornBush => (67, 57),
            PlantType::FlowerBush => (67, 57),
            PlantType::Vine => (67, 57),
            PlantType::Weed => (67, 57),
        }
    }
    pub fn sprite_index(&self) -> usize {
        let (row, col) = self.sprite_row_and_col();
        row * 64 + col
    }
    pub fn growth_speed(&self) -> f32 {
        match self {
            PlantType::Cabbage => 0.001,
            _ => 0.01
        }
    }
    pub fn is_forageable(&self) -> (Option<ItemType>, i32, ForageType) {
        match self {
            PlantType::Cabbage => (Some(ItemType::Cabbage), 1, ForageType::Once),
            PlantType::Carrot => (Some(ItemType::Carrot), 1, ForageType::Once),
            _ => (None, 0, ForageType::Once),
        }
    }
    pub fn is_choppable(&self) -> (Option<ItemType>, i32) {
        match self {
            PlantType::PineTree => (Some(ItemType::PineLog), 1),
            PlantType::OakTree => (Some(ItemType::OakLog), 1),
            PlantType::CedarTree => (Some(ItemType::CedarLog), 1),
            _ => (None, 0),
        }
    }
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum ForageType {
    Once, Repeat
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum SelectableType {
    Choppable,
    Constructable,
    Foragable,
    Gatherable,
    Harvestable,
    Mineable,
    Nothing,
    Unselecting,
    Unzoning,
    Zoning,
}

#[derive(Component)]
pub struct WorkTarget;

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
    pub unreachable: bool,
    pub moving_target: bool,
}

impl Default for Pathing {
    fn default() -> Self {
        Pathing {
            path: Vec::new(),
            destination: Position { x: 0, y: 0, z: 0 },
            unreachable: false,
            moving_target: false,
        }
    }
}

#[derive(Component)]
pub struct TemporaryVisualElement {
    pub duration: f32
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

#[derive(Component, Clone, Copy)]
pub struct Need {
    pub current: f32,
    pub max: f32,
    pub rate: f32,
    pub low: f32,
    pub normal: f32,
    pub high: f32,
}

#[derive(Component)]
#[derive(Default)]
pub struct Logs;

