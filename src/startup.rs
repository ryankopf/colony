use bevy::prelude::*;
use super::components::{Position, SizeXYZ};
use super::prelude::*;

// Make Startup Plugin
pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(startup)
        .insert_resource(MyFont(Handle::<Font>::default()))
        ;
    }
}

pub fn startup(
    mut commands: Commands,
    sprite_sheet: Res<SpriteSheet>,
    _asset_server: Res<AssetServer>,
    biome: Res<Biome>,
) {
    
    // GENERATE UNITS
    for i in 1..6 {
        let position = Position { x: 3, y: 3*i, z: 0 };
        let sprite =  TextureAtlasSprite::new(ActorType::Human.sprite_index()); // TO DO
        
        commands.spawn(SpriteSheetBundle {
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
        .insert(position.to_transform_layer(1.0))
        .insert(Attackable)
        // .insert(NeedsFood { current: 100.0, max: 100.0, rate: 0.1 })
        .insert( GiveMeAName )
        .insert( Status {
            needs_food: Some(NeedsFood { current: 25.1, max: 100.0, rate: 0.1 }),
            needs_entertainment: Some(NeedsEntertainment { current: 100.0, max: 100.0, rate: 0.1 }),
            needs_sleep: Some(NeedsSleep { current: 15.2, max: 100.0, rate: 0.1 }),
            index: 0,
            crisis: None,
            danger: None,
            injured: false
        } )
        .insert( Brain { ..Default::default() } )
        ;
    }

    let position = Position { x: 10, y: 10, z: 0 };
    commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                ..default()
            })
            .insert(position)
            .insert(SizeXYZ::cube(1.1))
            .insert(super::components::MonsterGenerator)
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
