use super::prelude::*;

pub struct MainMenusPlugin;

impl Plugin for MainMenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(open_main_menu))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(close_main_menu));
    }
}

fn open_main_menu(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    _materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
    font: Res<MyFont>,
) {
    // Center window
    let window = windows.get_primary_mut().unwrap();
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
                position: UiRect {
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..Default::default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            background_color: Color::rgba(0.65, 1.0, 0.65, 0.65).into(),
            ..Default::default()
        })
        .insert(MainMenuOverlay)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section("WELCOME TO", text_style.clone()).with_alignment(
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section("COLONY", text_style.clone())
                    .with_alignment(TextAlignment::TOP_CENTER),
                ..default()
            });
            parent.spawn(
                TextBundle::from_section("Get Started", text_style.clone()).with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );
            // Next insert a button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(100.0), Val::Px(30.0)),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section("Start Game", text_style.clone())
                            .with_text_alignment(TextAlignment::CENTER),
                    );
                });
        });
}

pub fn close_main_menu(
    mut commands: Commands,
    _state: ResMut<State<GameState>>,
    mut query: Query<Entity, With<MainMenuOverlay>>,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}
