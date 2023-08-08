use crate::prelude::*;

// Create plugin.
pub struct InfoPanelPlugin;

impl Plugin for InfoPanelPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            Update,
            (show_info_panel, info_system)
        )
        ;
    }
}

pub fn show_info_panel(
    mut commands: Commands,
    font: Res<MyFont>,
    object_info: Res<InfoPanelInformation>,
    texts: Query<Entity, With<InfoPanelText>>,
) {
    for text in texts.iter() {
        commands.entity(text).despawn();
    }
    commands.spawn((
        TextBundle::from_section(
            &object_info.name,
            TextStyle {
                font: font.0.clone(),
                font_size: 24.0,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(15.0),
            left: Val::Px(15.0),
            ..default()
        }),
        InfoPanelText,
    ));
    for (i, info) in object_info.info.iter().enumerate() {
        commands.spawn((
            TextBundle::from_section(
                info,
                TextStyle { font: font.0.clone(), ..default() },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::Left)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(45.0 + (i as f32 * 20.0)),
                left: Val::Px(15.0),
                ..default()
            }),
            InfoPanelText,
        ));
    }
}

pub fn info_system(
    mut commands: Commands,
    mut people: Query<(Entity, &Position, &Brain, &PhysicalBody, Option<&HasName>), With<ClickedOn>>,
    mut info_panel: ResMut<InfoPanelInformation>,
) {
    if let Some((_, position, brain, physical_body, has_name)) = people.iter_mut().last() {
        if let Some(has_name) = has_name {
            info_panel.name = has_name.name.clone();
        } else {
            info_panel.name = String::from("");
        }
        info_panel.info = vec![];
        info_panel.info.push(format!("Position: {}, {}", position.x, position.y));
        info_panel.info.extend_from_slice(&physical_body.info_panel_needs());
        info_panel.info.extend_from_slice(&brain.info_panel());
        info_panel.needs.extend_from_slice(&physical_body.info_panel_needs());
        info_panel.attributes.extend_from_slice(&physical_body.info_panel_attributes());
        info_panel.skills.extend_from_slice(&physical_body.info_panel_skills());
    }
    let count = people.iter().count();
    for (index, (entity, _, _, _, _)) in people.iter_mut().enumerate() {
        if index < count - 1 {
            commands.entity(entity).remove::<ClickedOn>();
        }
    }
}

#[derive(Component)]
pub struct InfoPanelText;