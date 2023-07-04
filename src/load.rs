use super::prelude::*;

pub fn load_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // let texture_handle = asset_server.load("sprites.png");
    let texture_handle = asset_server.load("AllSprites.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 64, 95, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(SpriteSheet(texture_atlas_handle));
}

pub fn load_font(asset_server: Res<AssetServer>, mut font_handle: ResMut<MyFont>) {
    *font_handle = MyFont(asset_server.load("fonts/Helvetica.ttf"));
}
