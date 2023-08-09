use crate::prelude::*;
pub mod territorial;
pub mod nopersonality;

pub struct PersonalityPlugin;

impl Plugin for PersonalityPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            Update,
                (territorial::personality_territorial, nopersonality::personality_nopersonality)
                .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
                .run_if(in_state(GameState::InGame))
        );
    }
}
