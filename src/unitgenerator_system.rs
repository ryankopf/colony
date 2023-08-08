use crate::prelude::*;

// Make plugin
pub struct UnitGeneratorPlugin;

impl Plugin for UnitGeneratorPlugin {
    fn build(&self, _app: &mut App) {
        
    }
}

pub fn spawn_unit_from_template(
    commands: &mut Commands,
    position: Position,
    sprite_sheet: &Res<SpriteSheet>,
    template: &UnitTemplate,
) -> Entity {
    let sprite =  TextureAtlasSprite::new(template.actor_type.sprite_index());
    let entity = commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: sprite_sheet.0.clone(),
            ..default()
        })
        .insert(position)
        .insert(position.to_transform_layer(1.0))
        .insert( PhysicalBody {
            needs_food: template.food_need.map(Into::into),
            needs_entertainment: template.entertainment_need.map(Into::into),
            needs_sleep: template.sleep_need.map(Into::into),
            index: 0,
            crisis: None,
            danger: None,
            injured: false,
            afflictions: template.afflictions.clone(),//Vec::new(),
            skillset: template.skillset.clone(),
            attributes: template.attributes.clone(),
        } )
        .insert( Brain {
            personality: template.personality.clone(),
            ..default()
        } )
        .id();
    for builder in &template.component_builders {
        builder(commands, entity);
    };
    entity
}

type ComponentBuilder = fn(&mut Commands, Entity);

pub struct UnitTemplate {
    pub actor_type: ActorType,
    pub food_need: Option<NeedExample>,
    pub entertainment_need: Option<NeedExample>,
    pub sleep_need: Option<NeedExample>,
    pub personality: Vec<PersonalityTrait>,
    pub skillset: Skillset,
    pub attributes: Attributeset,
    pub afflictions: Vec<Affliction>,
    pub component_builders: Vec<ComponentBuilder>,
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
            food_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            entertainment_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            sleep_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            personality: vec![],
            skillset: Self::random_skillset_humanoid(),
            attributes: Self::random_attributeset_humanoid(),
            afflictions: random_afflictions.to_vec(),
            component_builders: vec![
                |commands: &mut Commands, entity: Entity| { commands.entity(entity).insert(GiveMeAName); },
            ],
        }
    }
    pub fn elf() -> Self {
        let actor_type = ActorType::Elf;
        let random_afflictions = Self::random_afflictions_humanoid();
        Self {
            actor_type,
            food_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            entertainment_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            sleep_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            personality: vec![],
            skillset: Self::random_skillset_humanoid(),
            attributes: Self::random_attributeset_humanoid(),
            afflictions: random_afflictions.to_vec(),
            component_builders: vec![
                |commands: &mut Commands, entity: Entity| { commands.entity(entity).insert(GiveMeAName); },
            ],
        }
    }
    pub fn dwarf() -> Self {
        let actor_type = ActorType::Dwarf;
        let random_afflictions = Self::random_afflictions_humanoid();
        Self {
            actor_type,
            food_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            entertainment_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            sleep_need: Some(NeedExample { current: 90.0, max: 100.0, rate: 0.1, low: 10.0, normal: 25.0, high: 80.0, variance: 5.0 }),
            personality: vec![],
            skillset: Self::random_skillset_humanoid(),
            attributes: Self::random_attributeset_humanoid(),
            afflictions: random_afflictions.to_vec(),
            component_builders: vec![
                |commands: &mut Commands, entity: Entity| { commands.entity(entity).insert(GiveMeAName); },
            ],
        }
    }
    pub fn rat() -> Self {
        let actor_type = ActorType::Rat;
        let random_afflictions = vec![];//Self::random_afflictions_animal();
        Self {
            actor_type,
            food_need: None,
            entertainment_need: None,
            sleep_need: None,
            personality: vec![PersonalityTrait::Creature, PersonalityTrait::Vicious],
            afflictions: random_afflictions.to_vec(),
            skillset: Skillset::default(),
            attributes: Attributeset::default(),
            component_builders: vec![
                |commands: &mut Commands, entity: Entity| { commands.entity(entity).insert(HasName { name: "Rat".to_string() }); },
            ],
        }
    }
    pub fn spider() -> Self {
        let actor_type = ActorType::Spider;
        let random_afflictions = vec![];//Self::random_afflictions_animal();
        Self {
            actor_type,
            food_need: None,
            entertainment_need: None,
            sleep_need: None,
            personality: vec![PersonalityTrait::Creature, PersonalityTrait::Vicious],
            afflictions: random_afflictions.to_vec(),
            skillset: Skillset::default(),
            attributes: Attributeset::default(),
            component_builders: vec![
                |commands: &mut Commands, entity: Entity| { commands.entity(entity).insert(HasName { name: "Spider".to_string() }); },
            ],
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
        let mut rng = rand::thread_rng();
        let ranges = [5000..7000, 5000..7000, 3000..4000, 2000..3000, 1000..2000, 500..1000, 500..1000, 400..800, 300..600, 300..600, 200..500, 200..500, 100..300];
        let mut values: Vec<i32> = ranges.iter().map(|range| rng.gen_range(range.clone())).collect();
        let losses = [1000..2000, 1000..2000, 200..500, 200..500, 100..300, 100..300, 100..300, 100..300, 100..300, 100..300, 100..300, 100..300, 100..300];
        let mut loss_values: Vec<i32> = losses.iter().map(|range| rng.gen_range(range.clone())).collect();
        values.shuffle(&mut rng);
        Skillset {
            animal_raising: Skill   { experience: values.pop().unwrap_or(100), exp_lost: loss_values.pop().unwrap_or(0) },
            brawling: Skill         { experience: values.pop().unwrap_or(100), exp_lost: loss_values.pop().unwrap_or(0) },
            construction: Skill     { experience: values.pop().unwrap_or(100), exp_lost: loss_values.pop().unwrap_or(0) },
            cooking: Skill          { experience: values.pop().unwrap_or(100), exp_lost: loss_values.pop().unwrap_or(0) },
            crafting: Skill         { experience: values.pop().unwrap_or(100), exp_lost: loss_values.pop().unwrap_or(0) },
            doctoring: Skill        { experience: values.pop().unwrap_or(100), exp_lost: loss_values.pop().unwrap_or(0) },
            farming: Skill          { experience: values.pop().unwrap_or(100), exp_lost: loss_values.pop().unwrap_or(0) },
            fishing: Skill          { experience: values.pop().unwrap_or(100), exp_lost: loss_values.pop().unwrap_or(0) },
            foraging: Skill         { experience: values.pop().unwrap_or(100), exp_lost: loss_values.pop().unwrap_or(0) },
            hunting: Skill          { experience: values.pop().unwrap_or(100), exp_lost: loss_values.pop().unwrap_or(0) },
            mining: Skill           { experience: values.pop().unwrap_or(100), exp_lost: loss_values.pop().unwrap_or(0) },
            social: Skill           { experience: values.pop().unwrap_or(100), exp_lost: loss_values.pop().unwrap_or(0) },
            woodcutting: Skill      { experience: values.pop().unwrap_or(100), exp_lost: loss_values.pop().unwrap_or(0) },
        }
    }
    
    pub fn random_attributeset_humanoid() -> Attributeset {
        Attributeset {
            health: 100,
            strength: 3,    // Represents ability to do physical work.
            dexterity: 3,   // Represents ability to do fine work, and affects speed.
            constitution: 3,// Represents ability to resist disease, poison, and damage.
            intelligence: 3,// Represents ability to learn and remember.
            wisdom: 3,      // Represents ability to make good decisions.
            charisma: 4,    // Represents ability to influence others.
        }
    }
}