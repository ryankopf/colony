use crate::{prelude::*, unitgenerator_system::spawn_unit_from_template};

// Make plugin
pub struct MonsterGeneratorPlugin;

impl Plugin for MonsterGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, monster_generator.run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
        );
    }
}

pub fn monster_generator(
    mut commands: Commands,
    entities: Query<(Entity, &Position, &MonsterGenerator)>,
    tile_types: Query<(&Position, &TileType)>,
    generated_monsters: Query<(Entity, &GeneratedBy)>,
    sprite_sheet: Res<SpriteSheet>,
) {
    for (entity, position, monster_generator) in entities.iter() {
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
            if *tile_position == p2 && !tile_type.is_wall() {
                can_generate = true;
            }
        }
        for (_ent, parent) in generated_monsters.iter() {
            if parent.entity == entity {
                can_generate = false;
            }
        }
        if !can_generate {
            return;
        }
        let monster = spawn_unit_from_template(&mut commands, new_position, &sprite_sheet, monster_generator.pick());
        commands.entity(monster).insert(GeneratedBy { entity });
        
        //*position = new_position;
    }

}