use super::prelude::*;

pub fn needs_food_system(
    mut query: Query<(&mut NeedsFood)>,
    mut notifier: EventWriter<FoodNotifEvent>,
) {
    for (mut needs_food) in query.iter_mut() {
        needs_food.current -= needs_food.rate;
        if needs_food.current == 5.0 {
            
        }
        if needs_food.current < 0.0 {
            needs_food.current = 0.0;
            
        }
    }
}
pub fn needs_food_notify(
    mut notif_reader: EventReader<FoodNotifEvent>,
) {
    for _ in notif_reader.iter() {
        println!("Someone needs food.");
    }
}
pub fn needs_status_system(
    mut query: Query<&mut Status>
) {
    for mut status in query.iter_mut() {
        if let Some(needs_food) = status.needs_food.as_mut() {
            needs_food.current -= needs_food.rate;
            if needs_food.current < 0.0 {
                needs_food.current = 0.0;
            }
        }
        if let Some(needs_entertainment) = status.needs_entertainment.as_mut() {
            needs_entertainment.current -= needs_entertainment.rate;
            if needs_entertainment.current < 0.0 {
                needs_entertainment.current = 0.0;
            }
        }
        if let Some(needs_sleep) = status.needs_sleep.as_mut() {
            needs_sleep.current -= needs_sleep.rate;
            if needs_sleep.current < 0.0 {
                needs_sleep.current = 0.0;
            }
        }
    }
}

pub struct FoodNotifEvent;
