use crate::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    // ! Images
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 2, rows = 1))]
    #[asset(path = "characters/bird/bird_idle.png")]
    pub bird_idle: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 6, rows = 1))]
    #[asset(path = "characters/bird/bird_run.png")]
    pub bird_run: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 5, rows = 1))]
    #[asset(path = "characters/bird/bird_jump.png")]
    pub bird_jump: Handle<TextureAtlas>,
    // ! Levels
    #[asset(path = "levels/test_level.ldtk")]
    pub test_level: Handle<LdtkAsset>,
    // ! UI
    #[asset(path = "ui/heart_empty.png")]
    pub heart_empty: Handle<Image>,
    #[asset(path = "ui/heart_full.png")]
    pub heart_full: Handle<Image>,
    // ! Fonts
    #[asset(path = "fonts/roboto.ttf")]
    pub default_font: Handle<Font>,
    // ! Enemies
    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 3, rows = 1))]
    #[asset(path = "characters/bushly/idle.png")]
    pub bushly_idle: Handle<TextureAtlas>,
    // ! Background
    #[asset(path = "background/back.png")]
    pub parallax_back: Handle<Image>,
    #[asset(path = "background/middle.png")]
    pub parallax_middle: Handle<Image>,
    #[asset(path = "background/far.png")]
    pub parallax_far: Handle<Image>,
}
pub fn load_assets_plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Start),
    );
    app.add_collection_to_loading_state::<_, MyAssets>(GameState::AssetLoading);
}
