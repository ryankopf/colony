use bevy::prelude::*;
use super::components::{Position, SizeXYZ};
use super::prelude::*;

pub fn startup(mut commands: Commands) {
    // GENERATE UNITS
    println!("Startup");
    for i in 0..2 {
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
            .insert(MoveRandom)
            .insert(position.to_transform_layer(1.0))
            .insert(Attackable)
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

pub fn setup_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "You have just arrived on an uncharted island.",
            TextStyle {
                font: asset_server.load("Helvetica.ttf"),
                font_size: 14.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::TOP_CENTER)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(15.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
    ));
}

#[derive(Component)]
struct ColorText;