use bevy::prelude::*;
use super::components::{Position, SizeXYZ};
use super::prelude::*;

pub fn startup(mut commands: Commands) {
    // GENERATE UNITS
    println!("Startup");
    for i in 1..3 {
        let position = Position { x: 3, y: 3*i, z: 0 };
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                ..default()
            })
            .insert(position)
            .insert(SizeXYZ::flat_2(TILE_SIZE+1.0))
            //.insert(MoveRandom)
            .insert(position.to_transform_layer(1.0))
            .insert(Attackable)
            .insert(NeedsFood { current: 100.0, max: 100.0, rate: 0.1 })
            .insert( GiveMeAName )
            .insert( Status {
                needs_food: Some(NeedsFood { current: 100.0, max: 100.0, rate: 0.1 }),
                needs_entertainment: Some(NeedsEntertainment { current: 100.0, max: 100.0, rate: 0.1 }),
                needs_sleep: Some(NeedsSleep { current: 5.2, max: 100.0, rate: 0.1 }),
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
    for i in 0..(MAP_WIDTH*MAP_LENGTH / 10) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(1..MAP_WIDTH-1);
        let y = rng.gen_range(1..MAP_LENGTH-1);
        let growth = rng.gen_range(0.1..1.0);
        let position = Position { x: x, y: y, z: 0 };
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
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: plant_color,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                ..default()
            })
            .insert(position)
            .insert(SizeXYZ::flat_2(TILE_SIZE+1.0))
            .insert(position.to_transform_layer(0.5))
            .insert(Plant { growth: growth, plant_type: plant_type })
            ;
    }

}



use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use winit::window::Icon;

pub fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    let primary = windows.get_window(WindowId::primary()).unwrap();

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/fort2.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    
    primary.set_title("Orc Fortress");
    primary.set_window_icon(Some(icon));
}