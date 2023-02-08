use super::prelude::*;

// Create plugin.
pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_game_ui);
    }
}

pub fn initialize_game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let sprite =  TextureAtlasSprite::new(190);
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..Default::default()
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Percent(100.0), Val::Px(32.0)),
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
                    position: UiRect {
                        left: Val::Px(32.0 * i as f32),
                        top: Val::Px(0.0),
                        ..Default::default()
                    },
                    size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                    ..Default::default()
                },
                image: asset_server.load(format!("i-{}.png",i)).into(),
                ..Default::default()
            });
        }
    });
}