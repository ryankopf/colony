use super::prelude::*;

// Make plugin.
pub struct ThinkingPlugin;

impl Plugin for ThinkingPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            Update,
            (
            thinking_system
            .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(0.5)))
            .run_if(in_state(GameState::InGame))
            ,
            remotivate_system
            .run_if(bevy::time::common_conditions::on_timer(bevy::utils::Duration::from_secs_f32(2.5)))
            .run_if(in_state(GameState::InGame))
            )
        )
        ;
    }
}

pub fn thinking_system(
    _commands: Commands,
    mut query: Query<(Entity, &mut Brain, &mut PhysicalBody)>,
) {
    for (_entity, mut brain, physical_body) in query.iter_mut() {
        // if does_thinking.thinking {
        //     continue;
        // }
        // does_thinking.thinking = true;
        // CHOOSE A MOTIVATION
        // 0. Is having a crisis.
        // 1. Has been given an order AND there's a danger or emergency.
        // Needs order: Food, Health, Sleep, Entertainment.
        // 2. Has been given an order AND there's a matching need.
        // Is sick or injured and is trying to go to the hospital.
        // 3. Has an urgently unmet need.
        // 4. Has been given an order.
        // Based on personality:
        // Has meaningful work available to do.
        // Has a desire.
        // Wants to socialize or be entertained.
        
        // THEN ASSIGN A TASK BASED ON THE MOTIVATION

        // First, if there's a crisis, make sure we're doing that.
        if let Some(_crisis) = &physical_body.crisis {
            if brain.motivation != Some(Motivation::Crisis) {
                brain.remotivate()
            }
        }
        // Next, if there's a danger, make sure we're doing that.
        if let Some(_danger) = &physical_body.danger {
            if brain.motivation != Some(Motivation::Danger) {
                brain.task = None;
                brain.motivation = None;
            }
        }

        if let Some(_m) = &brain.task {
            continue; // Already has a task.
        }
        if brain.motivation.is_none() {
            // #1 - Check for crisis and dangers.
            if let Some(_crisis) = &physical_body.crisis {
                brain.motivation = Some(Motivation::Crisis);
            // Process dangers.
            } else if let Some(_danger) = &physical_body.danger {
                if let Some(_order) = &brain.order {
                    brain.motivation = Some(Motivation::Order);
                } else {
                    brain.motivation = Some(Motivation::Danger);
                }
            // Process needs.
            }
            // FOOD
            if brain.motivation.is_none() {
                if let Some(n) = &physical_body.needs_food {
                    if n.current < n.low {
                        if let Some(order) = &brain.order {
                            if *order == Order::Eat {
                                brain.motivation = Some(Motivation::Order);
                            } else {
                                brain.motivation = Some(Motivation::Hunger);
                            }
                        } else {
                            brain.motivation = Some(Motivation::Hunger);
                        }
                    }
                }
            }
            // HOSPITAL
            if brain.motivation.is_none() && physical_body.injured {
                if let Some(order) = &brain.order {
                    if *order == Order::Hospital {
                        brain.motivation = Some(Motivation::Order);
                    } else {
                        brain.motivation = Some(Motivation::Injured);
                    }
                } else {
                    brain.motivation = Some(Motivation::Injured);
                }
            }
            // SLEEP
            if brain.motivation.is_none() {
                if let Some(n) = &physical_body.needs_sleep {
                    if n.current < n.low {
                        if let Some(order) = &brain.order {
                            if *order == Order::Sleep {
                                brain.motivation = Some(Motivation::Order);
                            } else {
                                brain.motivation = Some(Motivation::Tired);
                            }
                        } else {
                            brain.motivation = Some(Motivation::Tired);
                        }
                    }
                }
            }
            // ENTERTAINMENT
            if brain.motivation.is_none() {
                if let Some(n) = &physical_body.needs_entertainment {
                    if n.current < n.low {
                        if let Some(order) = &brain.order {
                            if *order == Order::Play {
                                brain.motivation = Some(Motivation::Order);
                            } else {
                                brain.motivation = Some(Motivation::Bored);
                            }
                        } else {
                            brain.motivation = Some(Motivation::Bored);
                        }
                    }
                }
            }
            // ORDERS
            if brain.motivation.is_none() {
                if let Some(_order) = &brain.order {
                    brain.motivation = Some(Motivation::Order);
                }
            }
            // Now decide what the unit should be doing based on its personality.
            if brain.motivation.is_none() {
                if brain.personality.contains(&PersonalityTrait::Creature) {
                    brain.motivation = Some(Motivation::Personality);
                } else {
                    // Work (75% chance) or Personality (25% chance)
                    // if random::<i32>() % 4 == 0 {
                    //     brain.motivation = Some(Motivation::Work);
                    // } else {
                    //     brain.motivation = Some(Motivation::Personality);
                    // }
                    brain.motivation = Some(Motivation::Personality);
                }
            }
        }
        if brain.motivation.is_none() {
            brain.motivation = Some(Motivation::Meander);
        }
        // ****************************************** //
        // ****************************************** //
        // NOW SET THE TASK RELATED TO THE MOTIVATION //
        if let Some(m) = brain.motivation {
            if m == Motivation::Crisis {
                if let Some(_crisis) = &physical_body.crisis {
                    // TO DO: Assign task based on crisis.
                    // if let Some(task) = &crisis.task {
                    //     brain.task = Some(task.to_string());
                    // }
                }
            } else if m == Motivation::Order {
                if let Some(_order) = &brain.order {
                    brain.task = Some(Task::Order);
                }
            } else if m == Motivation::Rage {
                brain.task = Some(Task::Fight);
            } else if m == Motivation::Danger {
                if let Some(danger) = &physical_body.danger {
                    match danger.danger_type {
                        DangerType::Attacked => {
                            brain.task = Some(Task::Fight);
                        },
                        DangerType::Fire => {
                            brain.task = Some(Task::Flee);
                        },
                        _ => {
                            brain.task = Some(Task::Flee);
                        },
                    }
                }
            } else if m == Motivation::Hunger {
                brain.task = Some(Task::Eat);
            } else if m == Motivation::Injured {
                brain.task = Some(Task::Hospital);
            } else if m == Motivation::Tired {
                brain.task = Some(Task::Sleep);
            } else if m == Motivation::Bored {
                brain.task = Some(Task::Play);
            } else if m == Motivation::Work {
                brain.task = Some(Task::Work);
            } else if m == Motivation::Personality {
                brain.task = Some(Task::Personality);
            } else if m == Motivation::Meander {
                brain.task = Some(Task::Meander);
            }
        }
        if let Some(_task) = &brain.task {
            //println!("{:?} has a task: {:?}", entity, brain.task)
        }
        
    }
}
pub fn remotivate_system(
    _commands: Commands,
    mut query: Query<(Entity, &mut Brain)>,
) {
    for (_entity, mut brain) in query.iter_mut() {
        if let Some(m) = brain.task {
            if m == Task::Work || m == Task::Play || m == Task::Meander {
                brain.motivation = None;
                brain.task = None;
            }
        }
    }
    
}