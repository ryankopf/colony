use super::prelude::*;
mod melee;
mod ranged;

pub const HALF_SECOND: &str = "half_second";

// Make Plugin
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_fixed_timestep_system(
                HALF_SECOND,
                0,
                melee::combat_system_melee.run_in_bevy_state(GameState::InGame),
            )
            .add_fixed_timestep_system(
                HALF_SECOND,
                0,
                ranged::combat_system_ranged.run_in_bevy_state(GameState::InGame),
            )
            ;
    }
}
