use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::GameState;

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 4, rows = 1))]
    #[asset(path = "test_art/bird.png")]
    pub bird: Handle<TextureAtlas>,
}

pub fn load_assets_plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Run),
    );
    app.add_collection_to_loading_state::<_, MyAssets>(GameState::AssetLoading);
}
