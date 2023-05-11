use bevy::prelude::*;
use nest_climb::{
    animate_sprites, camera_plugin, inputs_plugin, load_assets_plugin, move_player_system,
    physics_plugin, spawn_ground, spawn_player, GameState,
};
use seldom_fn_plugin::FnPluginExt;

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .fn_plugin(load_assets_plugin)
        .fn_plugin(camera_plugin)
        .fn_plugin(physics_plugin)
        .fn_plugin(inputs_plugin)
        .add_system(spawn_player.in_schedule(OnEnter(GameState::Run)))
        .add_system(spawn_ground.in_schedule(OnEnter(GameState::Run)))
        .add_system(animate_sprites)
        .add_system(move_player_system)
        .run();
}
