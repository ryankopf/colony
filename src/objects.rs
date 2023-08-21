use bevy::prelude::*;
use crate::prelude::*;

#[derive(Component)]
pub struct Object {
    pub itemtype: ItemType,
    pub remaining_resources: Vec<(ItemType, u8)>,
    pub under_construction: bool,
}
impl Default for Object {
    fn default() -> Self {
        Object {
            itemtype: ItemType::WallWood,
            remaining_resources: vec![],
            under_construction: false,
        }
    }
}

#[derive(Component)]
pub struct ItemReplacements {
    pub replacements: Vec<ItemType>,
}

#[derive(Clone, Copy)]
enum ItemGroup {
    Logs,
    Statues,
    Stones,
    Other,
    Walls,
}
impl ItemGroup {
    pub fn items(&self) -> Vec<ItemType> {
        match self {
            ItemGroup::Logs => vec![ItemType::CedarLog, ItemType::PineLog, ItemType::OakLog],
            ItemGroup::Stones => vec![ItemType::Goo1],
            ItemGroup::Walls => vec![ItemType::WallWood],
            ItemGroup::Statues => vec![ItemType::StatuePillar1,
                ItemType::StatuePillar2,
                ItemType::StatuePillar3,
                ItemType::StatuePillar4,
                ItemType::StatuePillar5,
                ItemType::StatuePillar6,
                ItemType::StatuePillar7,
                ItemType::StatueElephant,
                ItemType::StatueHead,
                ItemType::StatueIDK1,
                ItemType::StatueIDK2,
                ItemType::StatueGold,
                ItemType::StatueMan,
                ItemType::StatuePlatform,
                ItemType::StatueAnd,
                ItemType::StatueAt,
                ItemType::StatueAngel,
                ItemType::StatueArcher,
                ItemType::StatueHover,
                ItemType::StatueCat,
                ItemType::StatueCentaur,
                ItemType::StatueKing,
                ItemType::StatueKnight,
                ItemType::StatueDragon,
                ItemType::StatueDwarf,
                ItemType::StatueMastodon,
                ItemType::StatueHydra,
                ItemType::StatueSpearman,
                ItemType::StatueBall,
                ItemType::StatueBigfoot,
                ItemType::StatuePrincess,
                ItemType::StatueDeath,
                ItemType::StatueSnail,
                ItemType::StatueSword,
                ItemType::StatueDragon2,
                ItemType::StatueTriangle,
                ItemType::StatueWizard,
                ItemType::StatueGhost,],
            ItemGroup::Other => vec![],
        }
    }

    pub fn in_group(&self, item: &ItemType) -> bool {
        self.items().contains(item)
    }
    pub fn all() -> &'static [ItemGroup] {
        &[
            ItemGroup::Logs,
            ItemGroup::Stones,
            ItemGroup::Statues,
            ItemGroup::Walls,
        ]
    }
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum ItemType {
    CedarLog,
    PineLog,
    OakLog,
    StatuePillar1,
    StatuePillar2,
    StatuePillar3,
    StatuePillar4,
    StatuePillar5,
    StatuePillar6,
    StatuePillar7,
    StatueElephant,
    StatueHead,
    StatueIDK1,
    StatueIDK2,
    StatueGold,
    StatueMan,
    StatuePlatform,
    StatueAnd,
    StatueAt,
    StatueAngel,
    StatueArcher,
    StatueHover,
    StatueCat,
    StatueCentaur,
    StatueKing,
    StatueKnight,
    StatueDragon,
    StatueDwarf,
    StatueMastodon,
    StatueHydra,
    StatueSpearman,
    StatueBall,
    StatueBigfoot,
    StatuePrincess,
    StatueDeath,
    StatueSnail,
    StatueSword,
    StatueDragon2,
    StatueTriangle,
    StatueWizard,
    StatueGhost,
    Moss1,
    Moss2,
    Moss3,
    Moss4,
    Moss5,
    Goo1,
    Blood1,
    Blood2,
    Blood3,
    Blood4,
    Blood5,
    Goo2,
    LeafyDebris1,
    LeafyDebris2,
    LeafyDebris3,
    LeafyDebris4,
    WallBrick,
    WallMetal,
    WallStone,
    WallWood,
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

impl ItemType {
    pub fn sprite_row_and_col(&self) -> (usize, usize) {
        match self {
            ItemType::CedarLog => (94, 30),
            ItemType::PineLog => (94, 30),
            ItemType::OakLog => (94, 30),
            ItemType::StatuePillar1 => (19, 12),
            ItemType::StatuePillar2 => (19, 13),
            ItemType::StatuePillar3 => (19, 14),
            ItemType::StatuePillar4 => (19, 15),
            ItemType::StatuePillar5 => (19, 16),
            ItemType::StatuePillar6 => (19, 17),
            ItemType::StatuePillar7 => (19, 18),
            ItemType::StatueElephant => (19, 19),
            ItemType::StatueHead => (19, 20),
            ItemType::StatueIDK1 => (19, 21),
            ItemType::StatueIDK2 => (19, 22),
            ItemType::StatueGold => (19, 23),
            ItemType::StatueMan => (19, 24),
            ItemType::StatuePlatform => (19, 25),
            ItemType::StatueAnd => (19, 26),
            ItemType::StatueAt => (19, 27),
            ItemType::StatueAngel => (19, 28),
            ItemType::StatueArcher => (19, 29),
            ItemType::StatueHover => (19, 30),
            ItemType::StatueCat => (19, 31),
            ItemType::StatueCentaur => (19, 32),
            ItemType::StatueKing => (19, 33),
            ItemType::StatueKnight => (19, 34),
            ItemType::StatueDragon => (19, 35),
            ItemType::StatueDwarf => (19, 36),
            ItemType::StatueMastodon => (19, 37),
            ItemType::StatueHydra => (19, 38),
            ItemType::StatueSpearman => (19, 39),
            ItemType::StatueBall => (19, 40),
            ItemType::StatueBigfoot => (19, 41),
            ItemType::StatuePrincess => (19, 42),
            ItemType::StatueDeath => (19, 43),
            ItemType::StatueSnail => (19, 44),
            ItemType::StatueSword => (19, 45),
            ItemType::StatueDragon2 => (19, 46),
            ItemType::StatueTriangle => (19, 47),
            ItemType::StatueWizard => (19, 48),
            ItemType::StatueGhost => (19, 49),
            ItemType::Moss1 => (51, 3),
            ItemType::Moss2 => (51, 4),
            ItemType::Moss3 => (51, 5),
            ItemType::Moss4 => (51, 6),
            ItemType::Moss5 => (51, 7),
            ItemType::Goo1 => (51, 8),
            ItemType::Blood1 => (51, 9),
            ItemType::Blood2 => (51, 10),
            ItemType::Blood3 => (51, 11),
            ItemType::Blood4 => (51, 12),
            ItemType::Blood5 => (51, 13),
            ItemType::Goo2 => (51, 14),
            ItemType::LeafyDebris1 => (50, 15),
            ItemType::LeafyDebris2 => (50, 16),
            ItemType::LeafyDebris3 => (50, 17),
            ItemType::LeafyDebris4 => (50, 18),
            ItemType::WallBrick => (8, 0),
            ItemType::WallMetal => (1, 61),
            ItemType::WallStone => (3, 10),
            ItemType::WallWood => (6, 32),
            ItemType::Aloe => (67, 57),
            ItemType::Azalea => (67, 57),
            ItemType::Bush => (67, 57),
            ItemType::Cabbage => (94, 32),
            ItemType::CactusRound => (67, 57),
            ItemType::CactusUp => (67, 57),
            ItemType::Carrot => (94, 31),
            ItemType::CedarTree => (20, 14),
            ItemType::PineTree => (20, 15),
            ItemType::OakTree => (20, 16),
            ItemType::ThornBush => (67, 57),
            ItemType::FlowerBush => (67, 57),
            ItemType::Vine => (67, 57),
            ItemType::Weed => (67, 57),
        }
    }
    pub fn sprite_index(&self) -> usize {
        let (row, col) = self.sprite_row_and_col();
        row * 64 + col
    }
    pub fn growth_speed(&self) -> f32 {
        match self {
            ItemType::Cabbage => 0.001,
            _ => 0.01
        }
    }
    pub fn is_forageable(&self) -> (Option<ItemType>, i32, ForageType) {
        match self {
            ItemType::Cabbage => (Some(ItemType::Cabbage), 1, ForageType::Once),
            ItemType::Carrot => (Some(ItemType::Carrot), 1, ForageType::Once),
            _ => (None, 0, ForageType::Once),
        }
    }
    pub fn is_choppable(&self) -> (Option<ItemType>, i32) {
        match self {
            ItemType::PineTree => (Some(ItemType::PineLog), 1),
            ItemType::OakTree => (Some(ItemType::OakLog), 1),
            ItemType::CedarTree => (Some(ItemType::CedarLog), 1),
            _ => (None, 0),
        }
    }
    pub fn nutrition(&self) -> f32 {
        match self {
            ItemType::Cabbage => 10.0,
            ItemType::Carrot => 10.0,
            _ => 0.0,
        }
    }
    pub fn spoilage_rate(&self) -> f32 {
        match self {
            ItemType::Cabbage => 0.1,
            ItemType::Carrot => 0.1,
            _ => 0.01,
        }
    }
    pub fn carryable(&self) -> bool {
        match self.group() {
            ItemGroup::Statues => true,
            _ => false,
        }
    }
    pub fn passable(&self) -> bool {
        match self.group() {
            ItemGroup::Statues => false,
            ItemGroup::Walls => false,
            _ => {
                true
            }
        }
    }
    pub fn construction_needs(&self) -> Vec<(ItemType, u8)> {
        match self {
            ItemType::WallWood => vec![(ItemType::CedarLog, 10)],
            ItemType::Carrot => vec![],
            _ => {
                vec![]
            }
        }
    }
    pub fn potential_replacements(&self) -> Vec<ItemType> {
        self.group().items()
    }
    pub fn add_components(&self,
        commands: &mut Commands,
        entity: Entity
    ) {
        if self.carryable() {
            commands.entity(entity).insert( Carryable );
        }
    }

    fn group(&self) -> ItemGroup {
        for group in ItemGroup::all() {
            if group.in_group(self) {
                return *group;
            }
        }
        ItemGroup::Other
    }
}

impl Object {
    pub fn passable(&self) -> bool {
        self.itemtype.passable()
    }   
}
