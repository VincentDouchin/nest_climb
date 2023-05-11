use bevy::prelude::*;
use nest_climb::{animate_sprites, camera_plugin, load_assets_plugin, spawn_player, GameState};
use seldom_fn_plugin::FnPluginExt;
fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .fn_plugin(load_assets_plugin)
        .fn_plugin(camera_plugin)
        .add_system(spawn_player.in_schedule(OnEnter(GameState::Run)))
        .add_system(animate_sprites)
        .run();
}
