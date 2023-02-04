use bevy::prelude::*;


#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Component, PartialEq)]
pub enum TileType {
    Wall, Floor
}

#[derive(Component)]
pub struct Renderable {
    //pub glyph: rltk::FontCharType,
    pub fg: Color,
    pub bg: Color,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles : Vec<Position>,
    pub range : i32,
    pub dirty : bool
}

#[derive(Component)]
pub struct MapTile;

#[derive(Component)]
pub struct MoveRandom;

#[derive(Component)]
pub struct MonsterGenerator;

#[derive(Component)]
pub struct MoveTowardsNearestAttackable;

#[derive(Component)]
pub struct Attackable;

#[derive(Component)]
pub struct Mobile;

#[derive(Component)]
pub struct SizeXYZ {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}
impl SizeXYZ {
    pub fn cube(x: f32) -> Self {
        Self {
            width: x,
            height: x,
            depth: x,
        }
    }
}