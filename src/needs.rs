use crate::task_system::TWO_SECOND;
use super::prelude::*;

// Make plugin.
pub struct NeedsPlugin;

impl Plugin for NeedsPlugin {
    fn build(&self, app: &mut App) {
        app.add_fixed_timestep_system(
            TWO_SECOND, 0,
            needs_status_system.run_in_bevy_state(GameState::InGame),
        )
        ;
    }
}

pub fn needs_status_system(
    mut query: Query<&mut PhysicalBody>
) {
    for mut physical_body in query.iter_mut() {
        if let Some(needs_food) = physical_body.needs_food.as_mut() {
            needs_food.current -= needs_food.rate;
            if needs_food.current < 0.0 {
                needs_food.current = 0.0;
            }
        }
        if let Some(needs_entertainment) = physical_body.needs_entertainment.as_mut() {
            needs_entertainment.current -= needs_entertainment.rate;
            if needs_entertainment.current < 0.0 {
                needs_entertainment.current = 0.0;
            }
        }
        if let Some(needs_sleep) = physical_body.needs_sleep.as_mut() {
            needs_sleep.current -= needs_sleep.rate;
            if needs_sleep.current < 0.0 {
                needs_sleep.current = 0.0;
            }
        }
    }
}

pub struct FoodNotifEvent;
