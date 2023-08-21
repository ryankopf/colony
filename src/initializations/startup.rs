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

    let position = Position { x: 30, y: 6, z: 0 };
    let sprite =  TextureAtlasSprite::new(TileType::Cave.sprite_index());
    commands
            .spawn(SpriteSheetBundle {
                sprite,
                texture_atlas: sprite_sheet.0.clone(),
                ..default()
            })
            .insert(position)
            .insert(SizeXYZ::cube(1.0))
            .insert(MonsterGenerator { monsters: vec![(UnitTemplate::rat(),1),(UnitTemplate::spider(),5),(UnitTemplate::cyclops(),1)] })
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
            .insert( Object { itemtype: plant_type, ..default() } )
            .id()
            ;
        if plant_type.is_forageable().0.is_some() && growth > 0.5 {
            commands.entity(plant).insert(Foragable);
        }
        if plant_type.is_choppable().0.is_some() && growth > 0.5 {
            commands.entity(plant).insert(Choppable);
        }
    }
    // Spawn Objects (Items)
    for _ in 0..(MAP_WIDTH*MAP_LENGTH / biome.objects_overall_scarcity) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(1..MAP_WIDTH-1);
        let y = rng.gen_range(1..MAP_LENGTH-1);
        let position = Position { x, y, z: 0 };
        if taken_positions.contains_key(&position) { continue; }
        taken_positions.insert(position, 1);
        let object_type = biome.objects[rng.gen_range(0..biome.objects.len())];
        let sprite =  TextureAtlasSprite::new(object_type.sprite_index());
        
        let object = commands
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
            .insert( Object { itemtype: object_type, ..default() } )
            .id()
            ;
        object_type.add_components(&mut commands, object);
    }
}
