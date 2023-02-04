use bevy::prelude::*;
use super::constants::{COLOR_BLUE, COLOR_GRAY, COLOR_GREEN, COLOR_BLACK};
use super::components::{Position, SizeXYZ};

pub fn startup(mut commands: Commands) {
    // GENERATE UNITS
    for i in 0..20 {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_BLUE,
                    ..default()
                },
                ..default()
            })
            .insert(Position { x: 3, y: 3*i, z: 1 })
            .insert(SizeXYZ::cube(1.1))
            .insert(super::components::MoveRandom);
    }

    commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_BLACK,
                    ..default()
                },
                ..default()
            })
            .insert(Position { x: 10, y: 10, z: 1 })
            .insert(SizeXYZ::cube(1.1))
            .insert(super::components::MonsterGenerator)
            ;
}