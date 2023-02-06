use super::prelude::*;

pub fn thinking_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain, &mut Status)>,
) {
    for (entity, mut brain, mut status) in query.iter_mut() {
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
        if let Some(m) = &brain.task {
            continue; // Already has a task.
        }
        if let None = brain.motivation {
            // SET MOTIVATION
            if let Some(crisis) = &status.crisis {
                brain.motivation = Some("Crisis".to_string());
            // Process dangers.
            } else if let Some(danger) = &status.danger {
                if let Some(order) = &brain.order {
                    brain.motivation = Some("Order".to_string());
                } else {
                    brain.motivation = Some("Danger".to_string());
                }
            // Process needs.
            }
            // FOOD
            if let None = brain.motivation {
                if let Some(n) = &status.needs_food {
                    if n.current < 5.0 {
                        if let Some(order) = &brain.order {
                            if order == "Eat" {
                                brain.motivation = Some("Order".to_string());
                            } else {
                                brain.motivation = Some("Eat".to_string());
                            }
                        } else {
                            brain.motivation = Some("Eat".to_string());
                        }
                    }
                }
            }
            // HOSPITAL
            if let None = brain.motivation {
                if status.injured {
                    if let Some(order) = &brain.order {
                        if order == "Hospital" {
                            brain.motivation = Some("Order".to_string());
                        } else {
                            brain.motivation = Some("Hospital".to_string());
                        }
                    } else {
                        brain.motivation = Some("Hospital".to_string());
                    }
                }
            }
            // SLEEP
            if let None = brain.motivation {
                if let Some(n) = &status.needs_sleep {
                    if n.current < 5.0 {
                        if let Some(order) = &brain.order {
                            if order == "Sleep" {
                                brain.motivation = Some("Order".to_string());
                            } else {
                                brain.motivation = Some("Sleep".to_string());
                            }
                        } else {
                            brain.motivation = Some("Sleep".to_string());
                        }
                    }
                }
            }
            // ENTERTAINMENT
            if let None = brain.motivation {
                if let Some(n) = &status.needs_entertainment {
                    if n.current < 5.0 {
                        if let Some(order) = &brain.order {
                            if order == "Entertainment" {
                                brain.motivation = Some("Order".to_string());
                            } else {
                                brain.motivation = Some("Entertainment".to_string());
                            }
                        } else {
                            brain.motivation = Some("Entertainment".to_string());
                        }
                    }
                }
            }
            // ORDERS
            if let None = brain.motivation {
                if let Some(order) = &brain.order {
                    brain.motivation = Some("Order".to_string());
                }
            }
            // MEANINGFUL WORK
            if let None = brain.motivation {
                brain.motivation = Some("Work".to_string());
            }
        }
        if let None = brain.motivation {
            brain.motivation = Some("Meander".to_string());
        }
        // SET TASK
        if let Some(m) = &brain.motivation {
            if m == "Crisis" {
                if let Some(crisis) = &status.crisis {
                    // TO DO: Assign task based on crisis.
                    // if let Some(task) = &crisis.task {
                    //     brain.task = Some(task.to_string());
                    // }
                }
            } else if m == "Order" {
                if let Some(order) = &brain.order {
                    brain.task = Some(Task::Order);
                }
            } else if m == "Danger" {
                if let Some(danger) = &status.danger {
                    brain.task = Some(Task::Flee);
                    // TO DO: Assign FLEE or FIGHT task.
                }
            } else if m == "Eat" {
                brain.task = Some(Task::Eat);
                // Set task target.
                // commands.entity(entity).insert(TaskTarget::Food);
            } else if m == "Hospital" {
                brain.task = Some(Task::Hospital);
            } else if m == "Sleep" {
                brain.task = Some(Task::Sleep);
            } else if m == "Entertainment" {
                brain.task = Some(Task::Play);
            } else if m == "Work" {
                brain.task = Some(Task::Work);
            } else if m == "Meander" {
                brain.task = Some(Task::Meander);
            }
        }
        if let Some(task) = &brain.task {
            println!("{:?} has a task: {:?}", entity, brain.task)
        }
        
    }
}
pub fn remotivate_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Brain)>,
) {
    for (entity, mut brain) in query.iter_mut() {
        if let Some(m) = brain.task {
            if m == Task::Work || m == Task::Play || m == Task::Meander {
                brain.motivation = None;
                brain.task = None;
            }
        }
    }
    
}