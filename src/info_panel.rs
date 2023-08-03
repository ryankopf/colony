use super::prelude::*;

// Create plugin.
pub struct InfoPanelPlugin;

impl Plugin for InfoPanelPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(show_info_panel)
        .add_system(info_system)
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
    for (i, info) in object_info.info.iter().enumerate() {
        println!("Info: {}", info);
        commands.spawn((
            TextBundle::from_section(
                info,
                TextStyle { font: font.0.clone(), ..default() },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::TOP_LEFT)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(45.0 + (i as f32 * 20.0)),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
            InfoPanelText,
        ));
    }
}

pub fn info_system(
    mut commands: Commands,
    mut people: Query<(Entity, &Position, &Status, &ClickedOn)>,
    mut info_panel: ResMut<InfoPanelInformation>,
) {
    for (entity, position, status, _clickedon) in people.iter_mut() {
        commands.entity(entity).insert(ClickedOn);
        info_panel.info = vec![];
        info_panel.info.push(format!("Position: {}, {}", position.x, position.y));
        info_panel.info.extend_from_slice(&status.info_panel());
    }
}

#[derive(Component)]
pub struct InfoPanelText;