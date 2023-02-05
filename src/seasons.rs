use crate::prelude::*;

pub fn seasons(
    mut commands: Commands,
    mut plants: Query<(Entity, &mut Plant, &mut Transform)>,
) {
    for (entity, mut plant, mut transform) in plants.iter_mut() {
        if plant.growth < 1.0 {
            plant.growth += 0.03;
            transform.scale = Vec3::new(plant.growth, plant.growth, 1.0);
        } else {
            plant.growth += 0.01;
            if (plant.growth > 1.01) {
                let mut rng = rand::thread_rng();
                let death = rng.gen_range(0..100);
                if death < 2 {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}