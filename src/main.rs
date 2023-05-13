use bevy::prelude::*;
use bevy_tnua::*;
use leafwing_input_manager::prelude::*;
use nest_climb::{
    animate_sprites, camera_plugin, load_assets_plugin, move_player_system, physics_plugin,
    spawn_ground, spawn_player, update_direction, GameState, PlayerAction,
};
use seldom_fn_plugin::FnPluginExt;
fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(TnuaRapier2dPlugin)
        .add_plugin(TnuaPlatformerPlugin)
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .fn_plugin(load_assets_plugin)
        .fn_plugin(camera_plugin)
        .fn_plugin(physics_plugin)
        .add_system(spawn_player.in_schedule(OnEnter(GameState::Run)))
        .add_system(spawn_ground.in_schedule(OnEnter(GameState::Run)))
        .add_system(animate_sprites)
        .add_system(update_direction)
        .add_system(move_player_system)
        .run();
}
