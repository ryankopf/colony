use bevy::prelude::*;

pub const COLOR_BLACK: Color = Color::rgb(0.9, 0.9, 0.9);
pub const COLOR_GRAY: Color = Color::rgb(0.3, 0.3, 0.3);
pub const COLOR_GREEN: Color = Color::rgb(0.3, 0.9, 0.3);
pub const COLOR_RED: Color = Color::rgb(0.9, 0.3, 0.3);
pub const COLOR_BLUE: Color = Color::rgb(0.3, 0.3, 0.9);

// MAP CONSTANTS
pub const MAP_WIDTH: i32 = 160;
pub const MAP_LENGTH: i32 = 160;

// VIEW CONSTANTS
pub const VIEWAREA_WIDTH: u32 = 76;
pub const VIEWAREA_HEIGHT: u32 = 40;
pub const TILE_SIZE: u32 = 32;