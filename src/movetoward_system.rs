use crate::prelude::*;

pub fn movement_toward_attackable(
    //segments: ResMut<SnakeSegments>,
    mut entities: Query<(Entity, &mut MoveTowardsNearestAttackable, &mut Position, Without<TileType>)>,
    //mut positions: Query<&mut Position>,
    mut tile_types: Query<(&Position, &mut TileType)>,
) {

}