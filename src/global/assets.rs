use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::GameState;

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(texture_atlas(tile_size_x = 20.0, tile_size_y = 24.0, columns = 1, rows = 1))]
    #[asset(path = "characters/bird/bird01Grey.png")]
    pub bird: Handle<TextureAtlas>,
}

pub fn load_assets_plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Run),
    );
    app.add_collection_to_loading_state::<_, MyAssets>(GameState::AssetLoading);
}
