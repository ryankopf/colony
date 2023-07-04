use super::prelude::*;

pub fn namegiving_system(mut commands: Commands, mut query: Query<Entity, With<GiveMeAName>>) {
    for entity in query.iter_mut() {
        //name.name = "Bob".to_string();
        let names = vec![
            "Alice", "Charlie", "Dave", "Eve", "Frank", "Grace", "Hank", "Iris", "Judy", "Karl",
            "Linda", "Mike", "Nancy", "Oscar", "Peggy", "Quinn", "Ruth", "Steve", "Tina", "Ursula",
            "Victor", "Wendy", "Xavier", "Yvonne", "Zach",
        ];
        let i = rand::thread_rng().gen_range(0..names.len());
        let text_name = names[i];

        commands
            .entity(entity)
            .insert(HasName {
                name: text_name.to_string(),
            })
            .remove::<GiveMeAName>();
    }
}
