use crate::prelude::*;

// Create Plugin
pub struct SeasonsPlugin;

impl Plugin for SeasonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_timestep_system(crate::task_system::TWO_SECOND, 0, seasons.run_in_bevy_state(GameState::InGame));
    }
}

pub fn seasons(mut commands: Commands, mut plants: Query<(Entity, &mut Plant, &mut Transform, Option<&Foragable>, Option<&Choppable>)>) {
    for (entity, mut plant, mut transform, foragable, choppable) in plants.iter_mut() {
        if plant.growth < 1.0 {
            let rand = rand::thread_rng().gen_range(0..2);
            let base_growth_speed = plant.plant_type.growth_speed();
            plant.growth += match rand {
                0 => 3.0 * base_growth_speed,
                1 => base_growth_speed,
                _ => 0.0,
            };
            transform.scale = Vec3::new(plant.growth, plant.growth, 1.0);
            if plant.growth >= 0.5 {
                // Is plant one that is typically edible?
                if plant.plant_type.is_forageable().0.is_some() && foragable.is_none() {
                    commands.entity(entity).insert(Foragable);
                }
                if plant.plant_type.is_choppable().0.is_some() && choppable.is_none() {
                    commands.entity(entity).insert(Choppable);
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
