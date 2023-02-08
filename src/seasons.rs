use crate::prelude::*;

// Create Plugin
pub struct SeasonsPlugin;

impl Plugin for SeasonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.0))
                .with_system(seasons),
        );
    }
}

pub fn seasons(
    mut commands: Commands,
    mut plants: Query<(Entity, &mut Plant, &mut Transform, Option<&Foragable>)>,
) {
    for (entity, mut plant, mut transform, foragable) in plants.iter_mut() {
        if plant.growth < 1.0 {
            let rand = rand::thread_rng().gen_range(0..2);
            plant.growth += match rand { 0 => 0.03, 1 => 0.01, _ => 0.0 };
            transform.scale = Vec3::new(plant.growth, plant.growth, 1.0);
            if plant.growth >= 0.5 {
                // Is plant one that is typically edible?
                if plant.plant_type == PlantType::BerryBush {
                    if foragable.is_none() {
                        commands.entity(entity).insert(Foragable);
                    }
                }
            }
        } else {
            plant.growth += 0.01;
            if plant.growth > 1.01 {
                let mut rng = rand::thread_rng();
                let death = rng.gen_range(0..100);
                if death < 2 {
                    //commands.entity(entity).despawn();
                    plant.growth = 0.01;
                }
            }
        }
    }
}