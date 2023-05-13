use bevy::prelude::*;
use nest_climb::{
    animate_sprites, camera_plugin, debug_plugin, initialize_libraries, load_assets_plugin,
    move_player_system, spawn_ground, spawn_player, update_direction, GameState,
};
use seldom_fn_plugin::FnPluginExt;
fn main() {
    App::new()
        // ! Libraries
        .fn_plugin(initialize_libraries)
        // ! Debug
        .fn_plugin(debug_plugin)
        // ! States
        .add_state::<GameState>()
        // ! Assets
        .fn_plugin(load_assets_plugin)
        // !Camera
        .fn_plugin(camera_plugin)
        // ! Spawn entities
        .add_system(spawn_player.in_schedule(OnEnter(GameState::Run)))
        .add_system(spawn_ground.in_schedule(OnEnter(GameState::Run)))
        // ! Animation
        .add_system(animate_sprites)
        .add_system(update_direction)
        // ! Movement
        .add_system(move_player_system)
        .run();
}
