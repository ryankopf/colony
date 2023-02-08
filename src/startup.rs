use bevy::prelude::*;
use super::components::{Position, SizeXYZ};
use super::prelude::*;

pub fn startup(mut commands: Commands, sprite_sheet: Res<SpriteSheet>) {
    // GENERATE UNITS
    println!("Startup");
    for i in 1..3 {
        let position = Position { x: 3, y: 3*i, z: 0 };
        let mut sprite =  TextureAtlasSprite::new(9);
                    
        commands.spawn(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: sprite_sheet.0.clone(),
            transform: Transform::from_xyz(
                position.x as f32 * TILE_SIZE,
                position.y as f32 * TILE_SIZE,
                position.z as f32 * TILE_SIZE,
            ),
            ..Default::default()
        })
        .insert(position)
        .insert(SizeXYZ::flat_2(TILE_SIZE+1.0))
        //.insert(MoveRandom)
        .insert(position.to_transform_layer(1.0))
        .insert(Attackable)
        .insert(NeedsFood { current: 100.0, max: 100.0, rate: 0.1 })
        .insert( GiveMeAName )
        .insert( Status {
            needs_food: Some(NeedsFood { current: 5.1, max: 100.0, rate: 0.1 }),
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
        let position = Position { x: x, y: y, z: 0 };
        if taken_positions.contains_key(&position) { continue; }
        taken_positions.insert(position, 1);
        let plant_type = match rng.gen_range(0..20) {
            0 => PlantType::BerryBush,
            1 => PlantType::OakTree,
            _ => PlantType::PineTree,
        };
        let plant_color = match plant_type {
            PlantType::BerryBush => Color::PURPLE,
            PlantType::OakTree => Color::rgb(0.5, 0.3, 0.0),
            PlantType::PineTree => Color::rgb(0.4, 0.4, 0.1),
            _ => Color::DARK_GREEN,
        };
        let mut sprite =  TextureAtlasSprite::new(match plant_type {
            PlantType::BerryBush => 33,
            PlantType::OakTree => 30,
            PlantType::PineTree => 31,
            _ => 0,
        });
        
        let plant = commands
            .spawn(SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: sprite_sheet.0.clone(),
                transform: Transform::from_xyz(
                    position.x as f32 * TILE_SIZE,
                    position.y as f32 * TILE_SIZE,
                    position.z as f32 * TILE_SIZE,
                ),
                ..Default::default()
            })
            .insert(position)
            .insert(SizeXYZ::flat_2(TILE_SIZE+1.0))
            .insert(position.to_transform_layer(0.5))
            .insert(Plant { growth: growth, plant_type: plant_type })
            .id()
            ;
        if plant_type == PlantType::BerryBush && growth > 0.5 {
            commands.entity(plant).insert(Foragable);
        }
    }

}

pub fn load_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 10, 20, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(SpriteSheet(texture_atlas_handle));
}