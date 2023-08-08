use crate::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;

pub fn set_window_title(
    // we have to use `NonSend` here
    // windows: NonSend<WinitWindows>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary = windows.single_mut();//.get_window(WindowId::primary()).unwrap();
    primary.title = "Colony".to_string();
}
pub fn set_window_icon(
    // we have to use `NonSend` here
    winit_windows: NonSend<WinitWindows>,
    primary_windows: Query<(Entity, &mut Window), With<PrimaryWindow>>,
) {
    let window = primary_windows.single();
    let primary = winit_windows.get_window(window.0).unwrap();
    
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/fort2.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = winit::window::Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    primary.set_window_icon(Some(icon));
}

pub fn set_window_maximized(
    mut primary_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut p = primary_windows.single_mut();
    p.set_maximized(true);
}