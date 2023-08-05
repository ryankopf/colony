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

// pub fn spawn_unit(
//     commands: &mut Commands,
//     position: Position,
//     sprite_sheet: &Res<SpriteSheet>,
//     actor_type: ActorType,
//     food_need: f32,
//     entertainment_need: f32,
//     sleep_need: f32,
// ) {
//     let sprite =  TextureAtlasSprite::new(actor_type.sprite_index());
//     commands
//         .spawn(SpriteSheetBundle {
//             sprite,
//             texture_atlas: sprite_sheet.0.clone(),
//             ..default()
//         })
//         .insert(position)
//         .insert(position.to_transform_layer(1.0))
//         .insert(Attackable)
//         .insert( GiveMeAName )
//         .insert( PhysicalBody {
//             needs_food: Some(Need { current: food_need, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0 }),
//             needs_entertainment: Some(Need { current: entertainment_need, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0 }),
//             needs_sleep: Some(Need { current: sleep_need, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0 }),
//             index: 0,
//             crisis: None,
//             danger: None,
//             injured: false,
//             afflictions: Vec::new(),
//             skillset: Skillset::default(),
//             attributes: Attributeset::default(),
//         } )
//         .insert( Brain { ..default() } )
//         ;
// }

pub fn spawn_unit_from_template(
    commands: &mut Commands,
    position: Position,
    sprite_sheet: &Res<SpriteSheet>,
    template: &UnitTemplate,
) {
    let sprite =  TextureAtlasSprite::new(template.actor_type.sprite_index());
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
            needs_food: Some(template.food_need.into()),
            needs_entertainment: Some(template.entertainment_need.into()),
            needs_sleep: Some(template.sleep_need.into()),
            index: 0,
            crisis: None,
            danger: None,
            injured: false,
            afflictions: template.afflictions.clone(),//Vec::new(),
            skillset: template.skillset.clone(),
            attributes: template.attributes.clone(),
        } )
        .insert( Brain { ..default() } )
        ;
}

pub struct UnitTemplate {
    pub actor_type: ActorType,
    pub food_need: NeedExample,
    pub entertainment_need: NeedExample,
    pub sleep_need: NeedExample,
    pub afflictions: Vec<Affliction>,
    pub skillset: Skillset,
    pub attributes: Attributeset,
}
#[derive(Copy, Clone)]
pub struct NeedExample {
    pub current: f32,
    pub max: f32,
    pub rate: f32,
    pub low: f32,
    pub normal: f32,
    pub high: f32,
    pub variance: f32,
}
impl From<NeedExample> for Need {
    fn from(need_example: NeedExample) -> Self {
        let mut rng = rand::thread_rng();
        let variance = need_example.variance;

        // Calculate random adjustments for each field
        let current_adjustment = rng.gen_range(-variance..variance);
        let max_adjustment = rng.gen_range(-variance..variance);
        let rate_adjustment = rng.gen_range(-variance..variance);
        let low_adjustment = rng.gen_range(-variance..variance);
        let normal_adjustment = rng.gen_range(-variance..variance);
        let high_adjustment = rng.gen_range(-variance..variance);

        // Create the Need struct with the adjusted values
        Self {
            current: need_example.current + current_adjustment,
            max: need_example.max + max_adjustment,
            rate: need_example.rate + rate_adjustment,
            low: need_example.low + low_adjustment,
            normal: need_example.normal + normal_adjustment,
            high: need_example.high + high_adjustment,
        }
    }
}

impl UnitTemplate {
    pub fn human() -> Self {
        let actor_type = ActorType::Man;
        let random_afflictions = Self::random_afflictions_humanoid();
        Self {
            actor_type,
            food_need: NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 },
            entertainment_need: NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 },
            sleep_need: NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 },
            afflictions: random_afflictions.to_vec(),
            skillset: Self::random_skillset_humanoid(),
            attributes: Self::random_attributeset_humanoid(),
        }
    }
    pub fn random_afflictions_humanoid() -> Vec<Affliction> {
        ////////////////////////////
        // Select some Afflictions
        let random_affliction_pair = || {
            let affliction_locations = vec![
                AfflictionLocation::Head,
                AfflictionLocation::LeftArm,
                AfflictionLocation::RightArm,
                AfflictionLocation::LeftLeg,
                AfflictionLocation::RightLeg,
                AfflictionLocation::Torso,
            ];
            let affliction_types = vec![
                AfflictionType::BrokenBone,
                AfflictionType::Cut,
                AfflictionType::Disease,
                AfflictionType::Frostbite,
                AfflictionType::Infection
            ];
            let mut rng = rand::thread_rng();
            let i1 = rng.gen_range(0..affliction_types.len());
            let i2 = rng.gen_range(0..affliction_locations.len());
            (affliction_types[i1], affliction_locations[i2])
        };
        let a = random_affliction_pair();
        let b = random_affliction_pair();
        let random_affliction_pair = || {
            let affliction_locations = vec![
                AfflictionLocation::Heart,
                AfflictionLocation::Stomach,
                AfflictionLocation::Liver,
                AfflictionLocation::Spleen,
                AfflictionLocation::Kidneys,
                AfflictionLocation::Bladder,
                AfflictionLocation::Intestines,
                AfflictionLocation::Genitals,
            ];
            let affliction_types = vec![
                AfflictionType::Pain,
                AfflictionType::Inflammation,
                AfflictionType::Disease,
                AfflictionType::Wound,
            ];
            let mut rng = rand::thread_rng();
            let i1 = rng.gen_range(0..affliction_types.len());
            let i2 = rng.gen_range(0..affliction_locations.len());
            (affliction_types[i1], affliction_locations[i2])
        };
        let c = random_affliction_pair();
        let d = random_affliction_pair();
        let affliction_list = vec![
            Affliction {
                affliction_type: a.0,
                affliction_location: a.1,
                duration: 0,
                severity: 1,
                worsening: false,
            },
            Affliction {
                affliction_type: b.0,
                affliction_location: b.1,
                duration: 0,
                severity: 1,
                worsening: false,
            },
            Affliction {
                affliction_type: c.0,
                affliction_location: c.1,
                duration: 0,
                severity: 1,
                worsening: false,
            },
            Affliction {
                affliction_type: d.0,
                affliction_location: d.1,
                duration: 0,
                severity: 1,
                worsening: false,
            },
        ];
        let mut rng = rand::thread_rng();
        // Shuffle the affliction_list randomly
        let mut shuffled_afflictions = affliction_list.clone();
        shuffled_afflictions.shuffle(&mut rng);
        // Take a random number of afflictions (up to 2)
        let num_afflictions = rng.gen_range(0..=2);
        let selected_afflictions = &shuffled_afflictions[..num_afflictions];
        selected_afflictions.to_vec()
    }
    pub fn random_skillset_humanoid() -> Skillset {
        Skillset {
            animal_raising: Skill { experience: 100, exp_lost: 0 },
            construction: Skill { experience: 50, exp_lost: 10 },
            cooking: Skill { experience: 75, exp_lost: 5 },
            crafting: Skill { experience: 60, exp_lost: 8 },
            doctoring: Skill { experience: 90, exp_lost: 2 },
            farming: Skill { experience: 70, exp_lost: 7 },
            fishing: Skill { experience: 80, exp_lost: 6 },
            foraging: Skill { experience: 40, exp_lost: 12 },
            hunting: Skill { experience: 55, exp_lost: 9 },
            mining: Skill { experience: 65, exp_lost: 5 },
            social: Skill { experience: 30, exp_lost: 15 },
            woodcutting: Skill { experience: 85, exp_lost: 4 },
        }
    }
    
    pub fn random_attributeset_humanoid() -> Attributeset {
        Attributeset {
            health: 100,
            strength: 3,
            dexterity: 3,
            constitution: 3,
            intelligence: 3,
            wisdom: 3,
            charisma: 4,
        }
    }
}