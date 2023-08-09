use bevy::prelude::*;
use crate::components::{Position, SizeXYZ};
use crate::prelude::*;
use crate::spawn_unit_from_template;
use crate::UnitTemplate;

// Make Startup Plugin
pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, startup)
        .insert_resource(MyFont(Handle::<Font>::default()))
        ;
    }
}

pub fn startup(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    biome: Res<Biome>,
) {
    // GENERATE UNITS
    for i in 1..=5 {
        let position = Position { x: 3, y: 3*i, z: 0 };
        match i {
            1 => {
                spawn_unit_from_template(&mut commands, position, &sprite_sheet, &UnitTemplate::elf());
            },
            2 => {
                spawn_unit_from_template(&mut commands, position, &sprite_sheet, &UnitTemplate::dwarf());
            },
            _ => {
                spawn_unit_from_template(&mut commands, position, &sprite_sheet, &UnitTemplate::human());
            }
        }
    }
    // SPAWN MANDATORY RUSTACEANS
    for i in 1..=3 {
        let position = Position { x: 20, y: 3*i, z: 0 };
        spawn_unit_from_template(&mut commands, position, &sprite_sheet, &UnitTemplate::crab());
    }

    let position = Position { x: 10, y: 6, z: 0 };
    let sprite =  TextureAtlasSprite::new(TileType::Cave.sprite_index());
    commands
            .spawn(SpriteSheetBundle {
                sprite,
                texture_atlas: sprite_sheet.0.clone(),
                ..default()
            })
            .insert(position)
            .insert(SizeXYZ::cube(1.1))
            .insert(MonsterGenerator)
            .insert(position.to_transform_layer(1.0))
            ;

    // GENERATE PLANTS
    // List taken positions as a Hashmap
    let mut taken_positions: HashMap<Position, u8> = HashMap::new();
    for _ in 0..(MAP_WIDTH*MAP_LENGTH / 10) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(1..MAP_WIDTH-1);
        let y = rng.gen_range(1..MAP_LENGTH-1);
        let growth = rng.gen_range(0.1..1.0);
        let position = Position { x, y, z: 0 };
        if taken_positions.contains_key(&position) { continue; }
        taken_positions.insert(position, 1);
        let plant_type = biome.plants[rng.gen_range(0..biome.plants.len())];
        // let plant_color = match plant_type {
        //     PlantType::BerryBush => Color::PURPLE,
        //     PlantType::OakTree => Color::rgb(0.5, 0.3, 0.0),
        //     PlantType::PineTree => Color::rgb(0.4, 0.4, 0.1),
        //     _ => Color::DARK_GREEN,
        // };
        let sprite =  TextureAtlasSprite::new(plant_type.sprite_index());
        
        let plant = commands
            .spawn(SpriteSheetBundle {
                sprite,
                texture_atlas: sprite_sheet.0.clone(),
                transform: Transform::from_xyz(
                    position.x as f32 * TILE_SIZE,
                    position.y as f32 * TILE_SIZE,
                    position.z as f32 * TILE_SIZE,
                ),
                ..Default::default()
            })
            .insert(position)
            .insert(position.to_transform_layer(0.5))
            .insert(Plant { growth, plant_type })
            .id()
            ;
        if plant_type.is_forageable().0.is_some() && growth > 0.5 {
            commands.entity(plant).insert(Foragable);
        }
        if [PlantType::OakTree,PlantType::PineTree].contains(&plant_type) && growth > 0.5 {
            commands.entity(plant).insert(Choppable);
        }
    }

}
