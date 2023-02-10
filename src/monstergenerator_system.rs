use crate::prelude::*;

// Make plugin
pub struct MonsterGeneratorPlugin;

impl Plugin for MonsterGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(monster_generator),
        );
    }
}

pub fn monster_generator(
    mut commands: Commands,
    entities: Query<(Entity, &Position), With<MonsterGenerator>>,
    tile_types: Query<(&Position, &TileType)>,
    generated_monsters: Query<(Entity, &GeneratedBy)>,
) {
    for (entity, position) in entities.iter() {
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
        for (tile_position, tile_type) in tile_types.iter() {
            let mut p2 = new_position;
            p2.z = 0;
            if *tile_position == p2 {
                if *tile_type != TileType::Wall {
                    can_generate = true;
                }
            }
        }
        for (ent, parent) in generated_monsters.iter() {
            if parent.entity == entity {
                can_generate = false;
            }
        }

        if (!can_generate) {
            return;
        }
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                ..default()
            })
            .insert(new_position)
            .insert(SizeXYZ::cube(1.1))
            .insert(new_position.to_transform_layer(1.0))
            .insert(GeneratedBy { entity: entity })
            .insert(MoveTowardsNearestAttackable)
            //.insert( HasName { name: "Wolf".to_string() } )
            ;
        //*position = new_position;
    }

}