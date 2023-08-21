use crate::prelude::*;

pub struct MainMenusPlugin;

impl Plugin for MainMenusPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            OnEnter(GameState::MainMenu), 
            open_main_menu
        )
        .add_systems(
            OnExit(GameState::MainMenu), 
            close_main_menu
        )
        ;
    }
}

fn open_main_menu(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    _materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window>,
    font: Res<MyFont>,
) {
    // Center window
    let window = windows.single_mut();
    let width = window.width();
    let height = window.height();
    let _window_size = Vec2::new(width, height);

    // Create main menu
    let text_style = TextStyle {
        font: font.0.clone(),
        font_size: 18.0,
        color: Color::WHITE,
    };
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Px(width),
                height: Val::Px(height),
                ..Default::default()
            },
            background_color: Color::rgba(0.65, 1.0, 0.65, 0.65).into(),
            ..Default::default()
        }).insert(MainMenuOverlay)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("WELCOME TO", text_style.clone())
                .with_alignment(TextAlignment::Center),
                ..default()
            })
            ;
            parent.spawn(TextBundle {
                text: Text::from_section("COLONY", text_style.clone())
                .with_alignment(TextAlignment::Center),
                ..default()
            });
            parent.spawn(
                TextBundle::from_section("Get Started", text_style.clone())
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                })
            );
            // Next insert a button
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(30.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Start Game",
                    text_style.clone(),
                ).with_text_alignment(TextAlignment::Center)
                );
            });


        })
        // .spawn(TextComponents {
        //     text: Text {
        //         value: "Press Space to Start".to_string(),
        //         font: asset_server.load("fonts/Roboto-Bold.ttf"),
        //         style: TextStyle {
        //             font_size: 30.0,
        //             color: Color::rgb(0.9, 0.9, 0.9),
        //             ..Default::default()
        //         },
        //     },
        //     style: Style {
        //         size: Size::new(Val::Px(window_size.x), Val::Px(window_size.y)),
        //         position_type: PositionType::Absolute,
        //         position: Rect {
        //             left: Val::Px(0.0),
        //             top: Val::Px(0.0),
        //             ..Default::default()
        //         },
        //         ..Default::default()
        //     },
        //     ..Default::default()
        // })
        ;
}

pub fn close_main_menu(
    mut commands: Commands,
    _state: ResMut<State<GameState>>,
    mut query: Query<Entity, With<MainMenuOverlay>>,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
    // state.set(GameState::InGame).unwrap();
}