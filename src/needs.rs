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

pub struct FoodNotifEvent;
