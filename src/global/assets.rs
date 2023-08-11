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
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 5, rows = 1))]
    #[asset(path = "characters/bird/bird_run.png")]
    pub bird_run: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 1, rows = 1))]
    #[asset(path = "characters/bird/bird_jump_up.png")]
    pub bird_jump_up: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 1, rows = 1))]
    #[asset(path = "characters/bird/bird_jump_down.png")]
    pub bird_jump_down: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 4, rows = 1))]
    #[asset(path = "characters/bird/bird_zoom.png")]
    pub bird_zoom: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 7, rows = 1))]
    #[asset(path = "characters/bird/bird_hurt.png")]
    pub bird_hurt: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 20, rows = 1))]
    #[asset(path = "characters/bird/bird_death.png")]
    pub bird_death: Handle<TextureAtlas>,
    // ! Levels
    #[asset(path = "levels/test_level.ldtk")]
    pub test_level: Handle<LdtkAsset>,
    // ! UI
    #[asset(path = "ui/heart_empty.png")]
    pub heart_empty: Handle<Image>,
    #[asset(path = "ui/heart_full.png")]
    pub heart_full: Handle<Image>,
    // ! Frames
    #[asset(path = "ui/frame_small.png")]
    pub frame_small: Handle<Image>,
    #[asset(path = "ui/frame_small_selected.png")]
    pub frame_small_selected: Handle<Image>,
    #[asset(path = "ui/frame_big.png")]
    pub frame_big: Handle<Image>,
    #[asset(path = "ui/button_big.png")]
    pub button_big: Handle<Image>,
    #[asset(path = "ui/button_big_pressed.png")]
    pub button_big_pressed: Handle<Image>,
    #[asset(path = "ui/selector.png")]
    pub selector: Handle<Image>,
    // ! Buttons
    #[asset(path = "ui/buttons/button_left.png")]
    pub button_left: Handle<Image>,
    #[asset(path = "ui/buttons/button_left_pressed.png")]
    pub button_left_pressed: Handle<Image>,
    #[asset(path = "ui/buttons/button_right.png")]
    pub button_right: Handle<Image>,
    #[asset(path = "ui/buttons/button_right_pressed.png")]
    pub button_right_pressed: Handle<Image>,
    #[asset(path = "ui/buttons/button_down.png")]
    pub button_down: Handle<Image>,
    #[asset(path = "ui/buttons/button_down_pressed.png")]
    pub button_down_pressed: Handle<Image>,
    #[asset(path = "ui/buttons/button_pause.png")]
    pub button_pause: Handle<Image>,
    #[asset(path = "ui/buttons/button_pause_pressed.png")]
    pub button_pause_pressed: Handle<Image>,
    #[asset(path = "ui/buttons/button_up.png")]
    pub button_up: Handle<Image>,
    #[asset(path = "ui/buttons/button_up_pressed.png")]
    pub button_up_pressed: Handle<Image>,
    // ! Fonts
    #[asset(path = "fonts/monogram.ttf")]
    pub default_font: Handle<Font>,
    // ! Enemies
    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 3, rows = 1))]
    #[asset(path = "characters/bushly/idle.png")]
    pub bushly_idle: Handle<TextureAtlas>,
    // ! Items
    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 4, rows = 1))]
    #[asset(path = "items/coin.png")]
    pub coin: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 8.0, columns = 1, rows = 1))]
    #[asset(path = "items/nest.png")]
    pub nest: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 48.0, tile_size_y = 8.0, columns = 1, rows = 1))]
    #[asset(path = "items/platform.png")]
    pub platform: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 55.0, tile_size_y = 56.0, columns = 8, rows = 1))]
    #[asset(path = "items/sawblade.png")]
    pub sawblade: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 1, rows = 1))]
    #[asset(path = "items/spikyball.png")]
    pub spikyball: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 4, rows = 1))]
    #[asset(path = "items/heart.png")]
    pub heart: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 16.0, columns = 3, rows = 1))]
    #[asset(path = "items/trampoline.png")]
    pub trampoline: Handle<TextureAtlas>,
    #[asset(path = "items/deadleaf.png")]
    pub deadleaf: Handle<Image>,
    // ! Background
    #[asset(path = "background/background.png")]
    pub background: Handle<Image>,
    #[asset(path = "background/cloud_1.png")]
    pub cloud_1: Handle<Image>,
    #[asset(path = "background/cloud_2.png")]
    pub cloud_2: Handle<Image>,
    #[asset(path = "background/tree_1.png")]
    pub tree_1: Handle<Image>,
    #[asset(path = "background/tree_2.png")]
    pub tree_2: Handle<Image>,
    #[asset(path = "background/tree_3.png")]
    pub tree_3: Handle<Image>,
    #[asset(path = "background/tree_4.png")]
    pub tree_4: Handle<Image>,
    #[asset(path = "background/cloud_3.png")]
    pub cloud_3: Handle<Image>,

    // ! Title
    #[asset(path = "background/title_nest.png")]
    pub title_nest: Handle<Image>,
    #[asset(path = "background/title_text.png")]
    pub title_text: Handle<Image>,
}
pub fn load_assets_plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Start),
    );
    app.add_collection_to_loading_state::<_, MyAssets>(GameState::AssetLoading);
}
