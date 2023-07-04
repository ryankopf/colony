use super::prelude::*;

pub fn movement_random(
    mut entities: Query<(&mut Position, &mut Transform), (With<MoveRandom>, Without<TileType>)>,
    tile_types: Query<(&Position, &TileType)>,
) {
    for (mut position, mut transform) in entities.iter_mut() {
        let mut new_position = *position;
        let mut rng = rand::thread_rng();
        let dir = rng.gen_range(0..4);
        match dir {
            0 => new_position.y += 1,
            1 => new_position.y -= 1,
            2 => new_position.x -= 1,
            3 => new_position.x += 1,
            _ => {}
        }
        for (tile_position, tile_type) in tile_types.iter() {
            let mut p2 = new_position;
            p2.z = 0;
            if *tile_position == p2 && !tile_type.is_wall() {
                *position = new_position;
                transform.translation.x = new_position.x as f32 * TILE_SIZE;
                transform.translation.y = new_position.y as f32 * TILE_SIZE;
            }
        }
    }
}
