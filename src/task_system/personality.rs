use crate::prelude::*;
pub mod human;
pub mod nopersonality;
pub mod territorial;
pub mod vicious;

pub struct PersonalityPlugin;

impl Plugin for PersonalityPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            Update,
                personalities
                .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
                .run_if(in_state(GameState::InGame))
        );
    }
}

pub fn personalities(
    mut entities: Query<(Entity, &mut Brain, &mut PhysicalBody, &Position, Option<&Nest>, Option<&Targeting>)>,
    mut objects: Query<(Entity, Option<&Object>, Option<&Zone>, Option<&WorkTarget>, &Position)>,
    tilehash: Res<TileHash>,
    objects2: Query<(&Object, &Position)>,
    zones: Query<(&Zone, &Position)>,
) {
    let tiletypes: &std::collections::HashMap<Position, TileType> = &tilehash.hash;
    let obstacles = crate::collect_obstacles(objects2, zones, tiletypes);
    let potential_targets = entities.iter()
        .map(|(entity, _, _, position, _, _)| (entity, *position)) // Clone the Position data
        .collect::<Vec<(Entity, Position)>>();
    let already_targeted = entities
        .iter()
        .filter(|(_, _, _, _, _, targeting)| targeting.is_some())
        .map(|(_, _, _, _, _, targeting)| targeting.unwrap().target)
        .collect::<Vec<Entity>>();
    for (entity, mut brain, mut physical_body, position, nest, targeting) in entities.iter_mut() {
        if brain.task != Some(Task::Personality) { continue; }
        let next_trait = brain.get_next_personality_trait();
        match next_trait {
            Some(PersonalityTrait::Vicious) => {
                vicious::vicious(entity, brain, physical_body, position, nest, &potential_targets);
            }
            Some(PersonalityTrait::Territorial) => {
                territorial::territorial(entity, brain, physical_body, position, nest, &potential_targets);
            },
            Some(PersonalityTrait::Human) => {
                human::human(entity, brain, physical_body, position, &objects, &already_targeted, &obstacles, &tiletypes);
            },
            _ => {
                nopersonality::nopersonality(entity, brain, physical_body, position, nest);
            },
        }
    }
}

