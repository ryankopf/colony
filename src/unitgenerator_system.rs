use crate::prelude::*;

// Make plugin
pub struct UnitGeneratorPlugin;

impl Plugin for UnitGeneratorPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(1.0))
        //         .with_system(unit_generator),
        // );
    }
}

pub fn spawn_unit(
    commands: &mut Commands,
    position: Position,
    sprite_sheet: &Res<SpriteSheet>,
    actor_type: ActorType,
    food_need: f32,
    entertainment_need: f32,
    sleep_need: f32,
) {
    let sprite =  TextureAtlasSprite::new(actor_type.sprite_index()); // TO DO
    commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: sprite_sheet.0.clone(),
            ..default()
        })
        .insert(position)
        .insert(position.to_transform_layer(1.0))
        .insert(Attackable)
        .insert( GiveMeAName )
        .insert( PhysicalBody {
            needs_food: Some(NeedsFood { current: food_need, max: 100.0, rate: 0.1 }),
            needs_entertainment: Some(NeedsEntertainment { current: entertainment_need, max: 100.0, rate: 0.1 }),
            needs_sleep: Some(NeedsSleep { current: sleep_need, max: 100.0, rate: 0.1 }),
            index: 0,
            crisis: None,
            danger: None,
            injured: false,
            afflictions: Vec::new(),
            skillset: Skillset::default(),
            attributes: Attributeset::default(),
        } )
        .insert( Brain { ..Default::default() } )
        ;
}