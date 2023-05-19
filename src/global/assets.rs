use crate::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    // ! Images
    #[asset(texture_atlas(tile_size_x = 20.0, tile_size_y = 24.0, columns = 1, rows = 1))]
    #[asset(path = "characters/bird/bird01Grey.png")]
    pub bird: Handle<TextureAtlas>,
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
}

pub fn load_assets_plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::InitRun),
    );
    app.add_collection_to_loading_state::<_, MyAssets>(GameState::AssetLoading);
}
