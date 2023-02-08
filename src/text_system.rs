use super::prelude::*;

pub fn setup_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "You have just arrived on an uncharted island.",
            TextStyle {
                font: asset_server.load("fonts/Helvetica.ttf"),
                font_size: 14.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::TOP_LEFT)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(15.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
    ));
}


pub fn text_update_system(
    mut query: Query<&mut Text, With<FpsText>>
) {
    for mut text in &mut query {
        text.sections[1].value = format!("ZZZZZZ");
        // if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        //     if let Some(value) = fps.smoothed() {
        //         // Update the value of the second section
        //         text.sections[1].value = format!("{value:.2}");
        //     }
        // }
    }
}


#[derive(Component)]
pub struct ColorText;

#[derive(Component)]
pub struct FpsText;

////////////////////////////////////////////
use bevy::{
    prelude::*,
    text::{Text2dBounds},
};
#[derive(Component)]
struct AnimateTranslation;

#[derive(Component)]
struct AnimateRotation;

#[derive(Component)]
struct AnimateScale;

pub fn text_test(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("fonts/FiraSans-Medium.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 18.0,
        color: Color::WHITE,
    };
    let text_alignment_center = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    let text_alignment_top_left = TextAlignment {
        vertical: VerticalAlign::Top,
        horizontal: HorizontalAlign::Left,
    };
    // // Demonstrate changing translation
    // commands.spawn((
    //     Text2dBundle {
    //         text: Text::from_section("translation", text_style.clone())
    //             .with_alignment(text_alignment_center),
    //         ..default()
    //     },
    //     AnimateTranslation,
    // ));
    // // Demonstrate changing rotation
    // commands.spawn((
    //     Text2dBundle {
    //         text: Text::from_section("rotation", text_style.clone()).with_alignment(text_alignment_center),
    //         ..default()
    //     },
    //     AnimateRotation,
    // ));
    // // Demonstrate changing scale
    // commands.spawn((
    //     Text2dBundle {
    //         text: Text::from_section("scale", text_style).with_alignment(text_alignment_center),
    //         ..default()
    //     },
    //     AnimateScale,
    // ));
    // // Demonstrate text wrapping
    // let slightly_smaller_text_style = TextStyle {
    //     font,
    //     font_size: 42.0,
    //     color: Color::WHITE,
    // };
    // let box_size = Vec2::new(300.0, 200.0);
    // let box_position = Vec2::new(0.0, -250.0);
    // commands
    //     .spawn(SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::rgb(0.25, 0.25, 0.75),
    //             custom_size: Some(Vec2::new(box_size.x, box_size.y)),
    //             ..default()
    //         },
    //         transform: Transform::from_translation(box_position.extend(0.0)),
    //         ..default()
    //     })
    //     .with_children(|builder| {
    //         builder.spawn(Text2dBundle {
    //             text: Text {
    //                 sections: vec![TextSection::new(
    //                     "this text wraps in the box\n(Unicode linebreaks)",
    //                     slightly_smaller_text_style.clone(),
    //                 )],
    //                 alignment: text_alignment_top_left
    //             },
    //             text_2d_bounds: Text2dBounds {
    //                 // Wrap text in the rectangle
    //                 size: box_size,
    //             },
    //             // ensure the text is drawn on top of the box
    //             transform: Transform::from_translation(Vec3::Z),
    //             ..default()
    //         });
    //     });

    // let other_box_size = Vec2::new(300.0, 200.0);
    // let other_box_position = Vec2::new(320.0, -250.0);
    // commands
    //     .spawn(SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::rgb(0.20, 0.3, 0.70),
    //             custom_size: Some(Vec2::new(other_box_size.x, other_box_size.y)),
    //             ..default()
    //         },
    //         transform: Transform::from_translation(other_box_position.extend(0.0)),
    //         ..default()
    //     })
    //     .with_children(|builder| {
    //         builder.spawn(Text2dBundle {
    //             text: Text {
    //                 sections: vec![TextSection::new(
    //                     "this text wraps in the box\n(AnyCharacter linebreaks)",
    //                     slightly_smaller_text_style.clone(),
    //                 )],
    //                 alignment: text_alignment_top_left
    //             },
    //             text_2d_bounds: Text2dBounds {
    //                 // Wrap text in the rectangle
    //                 size: other_box_size,
    //             },
    //             // ensure the text is drawn on top of the box
    //             transform: Transform::from_translation(Vec3::Z),
    //             ..default()
    //         });
    //     });
}