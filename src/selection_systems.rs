use super::prelude::*;

// Make plugin.
pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<SelectionEvent>()
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
            .with_system(select_unselecting),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
            .with_system(select_foragables),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
            .with_system(select_choppables),
        )
        ;
    }
}

pub struct SelectionEvent;

pub fn select_foragables(
    mut commands: Commands,
    mut query: Query<(Entity, Option<&Foragable>), With<Highlighted>>,
    highlighteds: Query<Entity, With<Highlighted>>,
    highlightboxes: Query<Entity, With<HighlightBox>>,
    event: EventReader<SelectionEvent>,
    dragging: Res<Dragging>,
) {
    if event.is_empty() { return; }
    if dragging.looking_for != SelectableType::Foragable { return; }
    for (entity, foragable) in query.iter_mut() {
        if foragable.is_some() {
            commands.entity(entity).insert(WorkTarget);
        }
    }
    unhighlight(commands, highlighteds, highlightboxes);
}

pub fn select_choppables(
    mut commands: Commands,
    mut query: Query<(Entity, Option<&Choppable>), With<Highlighted>>,
    highlighteds: Query<Entity, With<Highlighted>>,
    highlightboxes: Query<Entity, With<HighlightBox>>,
    event: EventReader<SelectionEvent>,
    dragging: Res<Dragging>,
    font: Res<MyFont>,
) {
    if event.is_empty() { return; }
    if dragging.looking_for != SelectableType::Choppable { return; }
    for (entity, selection_reason) in query.iter_mut() {
        if selection_reason.is_some() {
            commands.entity(entity).insert(WorkTarget);
            let child = commands.spawn((
                Text2dBundle {
                    text: Text::from_section("X", TextStyle { font: font.0.clone(), ..default() })
                        .with_alignment(TextAlignment::CENTER),
                    ..default()
                },
                WorkMarker
            ))
            .insert(Transform::from_xyz(10.0, 20.0, 100.0)).id();
            commands.entity(entity).push_children(&[child]);
        }
    }
    unhighlight(commands, highlighteds, highlightboxes);
}

fn unhighlight(
    mut commands: Commands,
    mut highlighteds: Query<Entity, With<Highlighted>>,
    mut highlightboxes: Query<Entity, With<HighlightBox>>,
) {
    for entity in highlighteds.iter_mut() {
        commands.entity(entity).remove::<Highlighted>();
    }
    for (highlightbox) in highlightboxes.iter() {
        commands.entity(highlightbox).despawn();
    }
}

fn select_unselecting(
    mut commands: Commands,
    mut query: Query<Entity, (With<Highlighted>, With<WorkTarget>)>,
    workmarkers: Query<(Entity, &Parent), With<WorkMarker>>,
    highlightboxes: Query<(Entity, &Parent), With<HighlightBox>>,
    event: EventReader<SelectionEvent>,
    dragging: Res<Dragging>,
) {
    if event.is_empty() { return; }
    if dragging.looking_for != SelectableType::Unselecting { return; }
    for entity in query.iter_mut() {
        commands.entity(entity).remove::<WorkTarget>();
        for (workmarker, parent) in workmarkers.iter() {
            if parent.get() != entity { continue; }
            commands.entity(workmarker).despawn();
        }
        commands.entity(entity).remove::<Highlighted>();
    }
    for (highlightbox, parent) in highlightboxes.iter() {
        commands.entity(highlightbox).despawn();
    }
}