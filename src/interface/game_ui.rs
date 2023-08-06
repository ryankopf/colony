use crate::prelude::*;

// Create plugin.
pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(initialize_game_ui)
        .add_systems(
            OnEnter(GameState::InGame),
            start_game_ui
        )
        .add_system(
            game_ui_click
            .run_if(in_state(GameState::InGame))
        )
        // .add_system_set(
        //     SystemSet::on_enter(GameState::InGame)
        //         .with_system(start_game_ui),
        // )
        // // .add_startup_system(start_game_ui)
        // .add_system_set(
        //     SystemSet::on_update(GameState::InGame)
        //         .with_system(game_ui_click),
        // )
        ;
    }
}

pub fn initialize_game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let _sprite =  TextureAtlasSprite::new(190);
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            left: Val::Px(0.0),
            bottom: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(32.0),
            ..Default::default()
        },
        background_color: Color::rgba(0.65, 0.65, 0.65, 0.65).into(),
        ..Default::default()
    })
    .with_children(|parent| {
        // Add Icon
        for i in 0..6 {
            parent.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(32.0 * i as f32),
                    top: Val::Px(0.0),
                    width: Val::Px(32.0),
                    height: Val::Px(32.0),
                    ..default()
                },
                image: asset_server.load(format!("i-{}.png",i)).into(),
                ..default()
            });
        }
    });
}

pub fn start_game_ui(
    mut commands: Commands,
    font: Res<MyFont>,
    mut menu_state: ResMut<MenuState>,
    game_buttons: Query<Entity, With<InGameButton>>,
) {
    let text_style = TextStyle {
        font: font.0.clone(),
        font_size: 18.0,
        color: Color::WHITE,
    };
    let buttons = [vec![
        "TASKS",
        "FARM",
        "ZONE",
        "BUILD",
        "CRAFT",
    ],vec![ // tasks
        "BACK",
        "CLEAR",
        "CHOP",
        "FORAGE",
        "GATHER",
        "HUNT",
        "MINE",
    ],vec![ // farm
        "BACK",
        "NOTHING",
        "CABBAGE",
        "PINE",
        "OAK",
        "CEDAR",
    ],vec![ // zone
        "BACK",
        "NOTHING",
        "FISHING",
        "HOSPITAL",
        "PARTY",
        "MEETING",
    ],vec![ // build
        "BACK",
        "NOTHING",
        "WALL",
        "BED",
        "TABLE",
        "CHAIR",
    ],
    ]
    ;
    for button in game_buttons.iter() {
        commands.entity(button).despawn_recursive();
    }
    //println!("BUTTON: {:?}", buttons[menu_state.i]);
    if (buttons.len()-1) < menu_state.state.to_index() {
        menu_state.state = MenuStates::Home;
    }
    for (i, button_text) in buttons[menu_state.state.to_index()].iter().enumerate() {
        commands.spawn((ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(100.0 + 100.0 * i as f32),
                bottom: Val::Px(30.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(84.0),
                height: Val::Px(64.0),
                ..default()
            },
            background_color: Color::rgba(0.65, 0.65, 0.85, 0.65).into(),
            ..default()
        },InGameButton)).with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(button_text.to_owned(), text_style.clone() )
                .with_alignment(TextAlignment::Center),
                ..default()
            });
        })
        ;
    }
}

pub fn game_ui_click(
    commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mut windows: Query<&mut Window>,
    mut menu_state: ResMut<MenuState>,
    font: Res<MyFont>,
    game_buttons: Query<Entity, With<InGameButton>>,
    mut dragging: ResMut<Dragging>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = windows.single_mut();
        let wc = window.cursor_position();
        if let Some(wc) = wc {
            let y = window.height() - wc.y;
            if y > 30.0 && y < 92.0 {
                let button_index = (wc.x as i32 - 100) / 100;
                println!("BUTTON: {}", button_index);
                match menu_state.state {
                    MenuStates::Home => {
                        match button_index {
                            0 => { menu_state.state = MenuStates::Tasks },
                            1 => { menu_state.state = MenuStates::Farm },
                            2 => { menu_state.state = MenuStates::Zone },
                            3 => { menu_state.state = MenuStates::Build },
                            4 => { menu_state.state = MenuStates::Craft },
                            _ => { },
                        }
                    }
                    MenuStates::Tasks => { // chop, forage, gather, hunt, mine
                        match button_index {
                            0 => {
                                dragging.looking_for = SelectableType::Nothing;
                                menu_state.state = MenuStates::Home;
                            },
                            1 => {
                                dragging.looking_for = SelectableType::Unselecting;
                            },
                            2 => {
                                dragging.looking_for = SelectableType::Choppable;
                            },
                            3 => {
                                dragging.looking_for = SelectableType::Foragable;
                                dragging.zone_type = ZoneType::Farm;
                            },
                            4 => {
                                dragging.looking_for = SelectableType::Gatherable;
                                dragging.zone_type = ZoneType::Farm;
                            },
                            _ => { },
                        }
                    }
                    MenuStates::Farm => {
                        match button_index {
                            0 => {
                                dragging.looking_for = SelectableType::Nothing;
                                menu_state.state = MenuStates::Home;
                            },
                            1 => {
                                dragging.looking_for = SelectableType::Unzoning;
                            },
                            2 => {
                                dragging.looking_for = SelectableType::Zoning;
                                dragging.zone_type = ZoneType::Farm;
                                dragging.plant_type = PlantType::Cabbage;
                            },
                            3 => {
                                dragging.looking_for = SelectableType::Zoning;
                                dragging.zone_type = ZoneType::Farm;
                                dragging.plant_type = PlantType::PineTree;
                            },
                            4 => {
                                dragging.looking_for = SelectableType::Zoning;
                                dragging.zone_type = ZoneType::Farm;
                                dragging.plant_type = PlantType::OakTree;
                            },
                            5 => {
                                dragging.looking_for = SelectableType::Zoning;
                                dragging.zone_type = ZoneType::Farm;
                                dragging.plant_type = PlantType::CedarTree;
                            },
                            _ => { },
                        }
                    }
                    MenuStates::Zone => {
                        menu_state.state = MenuStates::Home;
                    }
                    MenuStates::Build => {
                        menu_state.state = MenuStates::Home;
                    }
                    MenuStates::Craft => {
                        menu_state.state = MenuStates::Home;
                    }
                }
                start_game_ui(commands, font, menu_state, game_buttons);
            }
        }
    }
}