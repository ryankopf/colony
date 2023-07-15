use crate::prelude::*;

pub fn text_system(
    mut commands: Commands,
    font: Res<MyFont>,
    object_info: Res<SelectedObjectInformation>,
    texts: Query<Entity, With<ObjectText>>,
) {
    for text in texts.iter() {
        commands.entity(text).despawn();
    }
    for (i, info) in object_info.info.iter().enumerate() {
        //println!("POS: {}", (i as f32 * 20.0) );
        commands.spawn((
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                info,
                TextStyle { font: font.0.clone(), ..default() },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::TOP_LEFT)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(45.0 + (i as f32 * 20.0)),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
            ObjectText,
        ));
    }
}


pub fn text_update_system(
    mut query: Query<&mut Text, With<FpsText>>
) {
    for mut text in &mut query {
        text.sections[1].value = "ZZZZZZ".to_string();
        // if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        //     if let Some(value) = fps.smoothed() {
        //         // Update the value of the second section
        //         text.sections[1].value = format!("{value:.2}");
        //     }
        // }
    }
}


#[derive(Component)]
pub struct ObjectText;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct AnimateRotation;

#[derive(Component)]
struct AnimateScale;

pub fn text_test(
    
) {
    
}