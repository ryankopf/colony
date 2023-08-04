use crate::prelude::*;

pub fn status_display_system (
    mut commands: Commands,
    mut query: Query<(Entity, &HasName, &mut PhysicalBody, &Brain, &Children)>,
    mut q_children: Query<(Entity, &Parent), With<TextName>>,
    asset_server: Res<AssetServer>
) {
    for (child, _) in q_children.iter_mut() {
        commands.entity(child).despawn();
    }
    for (entity, has_name, mut physical_body, brain, _children) in query.iter_mut() {
        // Pick the text value to show.
        //let y = commands.entity(entity).log_components();//::<HasName>();
        //let mut text_value = "FIRE".to_string();
        let mut vec_statuses: Vec<String> = vec![];
        vec_statuses.push(has_name.name.clone());
        if let Some(n) = &physical_body.needs_food {
            if n.current < 5.0 {
                vec_statuses.push("HUNGRY".to_string());
            }
        }
        if let Some(n) = &physical_body.needs_entertainment {
            if n.current < 5.0 {
                vec_statuses.push("BORED".to_string());
            }
        }
        if let Some(n) = &physical_body.needs_sleep {
            if n.current < 5.0 {
                vec_statuses.push("TIRED".to_string());
            }
        }
        if let Some(Task::Sleeping) = brain.task {
            vec_statuses.push("ZZZ...".to_string());
        }
        // for child in children {
        //     commands.entity(entity).remove_children(&[*child]);
        //     commands.entity(*child).despawn();
        // }
        if physical_body.index >= vec_statuses.len() {
            physical_body.index = 0;
        }
        let chosen_text = vec_statuses[physical_body.index].clone();
        physical_body.index += 1;
        
        // NOW SHOW THE TEXT
        let font = asset_server.load("fonts/FiraSans-Medium.ttf");
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 18.0,
            color: Color::WHITE,
        };
        let text_alignment_center = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        };
        let child = commands.spawn((
            Text2dBundle {
                text: Text::from_section(chosen_text, text_style.clone())
                    .with_alignment(text_alignment_center),
                ..default()
            },
            TextName
        ))
        .insert(Transform::from_xyz(0.0, 30.0, 100.0))
        .insert(IsName)
        .id()
        ;
        commands.entity(entity).push_children(&[child]);


        // let e = parent.get();
        //commands.entity(nametext).insert(Transform::from_xyz(300.0, 0.0, 100.0));
        // println!("{:?}", text.sections);
        // text.sections[0].value = "FIRE".to_string();
        // text.set_changed();
        //println!("{}", text.sections[0].value);
    }
}
