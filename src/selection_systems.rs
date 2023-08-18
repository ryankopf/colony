use super::prelude::*;

// Make plugin.
pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<SelectionEvent>()
        .add_systems(
            Update,
            (
                select_unselecting
                .run_if(in_state(GameState::InGame))
            ,
                select_foragables
                .run_if(in_state(GameState::InGame))
            ,
                select_choppables
                .run_if(in_state(GameState::InGame))
            ,
                select_zoning
                .run_if(in_state(GameState::InGame))
            ,
                select_unzoning
                .run_if(in_state(GameState::InGame))
            ,
                select_nothing
                .run_if(in_state(GameState::InGame))
            )
        )
        ;
    }
}

#[derive(Event)]
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
                        .with_alignment(TextAlignment::Center),
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

pub fn select_zoning(
    mut commands: Commands,
    query: Query<(Entity, Option<&MapTile>), With<Highlighted>>,
    zoned: Query<Entity, With<Zone>>,
    highlighteds: Query<Entity, With<Highlighted>>,
    highlightboxes: Query<Entity, With<HighlightBox>>,
    event: EventReader<SelectionEvent>,
    dragging: Res<Dragging>,
) {
    if event.is_empty() { return; }
    if dragging.looking_for != SelectableType::Zoning { return; }
    'outer: for (entity, selection_reason) in query.iter() {
        for zoned in zoned.iter() { if zoned == entity { continue 'outer; } } // Don't zone tiles that already have a zone.
        if selection_reason.is_some() {
            commands.entity(entity).insert(Zone { zone_type: dragging.zone_type, plant_type: Some(dragging.plant_type), ..default() } );
            let zonemarker = commands.spawn( (SpriteBundle {
                sprite: Sprite {
                        color: Color::rgba(0.8, 0.8, 1.0, 0.1),
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                transform: Transform::from_xyz(0.0, 0.0, 300.0),
                ..default()
            }, ZoneMarker))
            .id();
            commands.entity(entity).push_children(&[zonemarker]);
        }
    }
    unhighlight(commands, highlighteds, highlightboxes);
}

fn unhighlight(
    mut commands: Commands,
    highlighteds: Query<Entity, With<Highlighted>>,
    highlightboxes: Query<Entity, With<HighlightBox>>,
) {
    for entity in highlighteds.iter() {
        commands.entity(entity).remove::<Highlighted>();
    }
    for highlightbox in highlightboxes.iter() {
        commands.entity(highlightbox).despawn();
    }
}

fn select_unselecting(
    mut commands: Commands,
    highlighteds: Query<Entity, With<Highlighted>>,
    workmarkers: Query<(Entity, &Parent), With<WorkMarker>>,
    highlightboxes: Query<Entity, With<HighlightBox>>,
    event: EventReader<SelectionEvent>,
    dragging: Res<Dragging>,
) {
    if event.is_empty() { return; }
    if dragging.looking_for != SelectableType::Unselecting { return; }
    for entity in highlighteds.iter() {
        commands.entity(entity).remove::<WorkTarget>();
        for (workmarker, parent) in workmarkers.iter() {
            if parent.get() != entity { continue; }
            commands.entity(workmarker).despawn();
        }
    }
    unhighlight(commands, highlighteds, highlightboxes);
}

fn select_nothing(
    commands: Commands,
    highlighteds: Query<Entity, With<Highlighted>>,
    highlightboxes: Query<Entity, With<HighlightBox>>,
    event: EventReader<SelectionEvent>,
    dragging: Res<Dragging>,
) {
    if event.is_empty() { return; }
    if dragging.looking_for != SelectableType::Nothing { return; }
    unhighlight(commands, highlighteds, highlightboxes);
}

fn select_unzoning(
    mut commands: Commands,
    highlighteds: Query<Entity, With<Highlighted>>,
    zonemarkers: Query<(Entity, &Parent), With<ZoneMarker>>,
    highlightboxes: Query<Entity, With<HighlightBox>>,
    event: EventReader<SelectionEvent>,
    dragging: Res<Dragging>,
) {
    if event.is_empty() { return; }
    if dragging.looking_for != SelectableType::Unzoning { return; }
    for entity in highlighteds.iter() {
        commands.entity(entity).remove::<Zone>();
        for (zonemarker, parent) in zonemarkers.iter() {
            if parent.get() != entity { continue; }
            commands.entity(zonemarker).despawn();
        }
    }
    unhighlight(commands, highlighteds, highlightboxes);
}