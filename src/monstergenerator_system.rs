use crate::prelude::*;

pub fn monster_generator(
    mut commands: Commands,
    //segments: ResMut<SnakeSegments>,
    entities: Query<(Entity, &Position, With<MonsterGenerator>)>,
    //mut positions: Query<&mut Position>,
    mut tile_types: Query<(&Position, &mut TileType)>,
) {
    for (_, position, _) in entities.iter() {
        let mut new_position = *position;
        let dir = random::<i32>() % 4;
        match dir {
            0 => new_position.y += 1,
            1 => new_position.y -= 1,
            2 => new_position.x -= 1,
            3 => new_position.x += 1,
            _ => {}
        }
        let mut can_generate = false;
        for (tile_position, mut tile_type) in tile_types.iter_mut() {
            let mut p2 = new_position;
            p2.z = 0;
            if *tile_position == p2 {
                if *tile_type != TileType::Wall {
                    can_generate = true;
                }
            }
        }
        if (!can_generate) {
            return;
        }
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_RED,
                    ..default()
                },
                ..default()
            })
            .insert(new_position)
            .insert(SizeXYZ::cube(1.1))
            .insert(super::components::MoveRandom);
        //*position = new_position;
    }

}