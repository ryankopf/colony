use super::prelude::*;

pub fn on_pause(mut commands: Commands) {
    // Insert a transparent gray overlay to darken the screen.
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    ..Default::default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            background_color: Color::rgba(0.75, 0.55, 0.55, 0.35).into(),
            ..Default::default()
        })
        .insert(PauseOverlay);
}

pub fn on_unpause(mut commands: Commands, query: Query<Entity, With<PauseOverlay>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
