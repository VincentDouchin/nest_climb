use bevy::prelude::*;

use nest_climb::{
    animate_sprites, camera_plugin, debug_plugin, initialize_libraries, load_assets_plugin,
    map_plugin, move_player_system, spawn_ground, spawn_player, spawn_walls, update_direction,
    GameState,
};
use seldom_fn_plugin::FnPluginExt;
fn main() {
    App::new()
        // ! Libraries
        .fn_plugin(initialize_libraries)
        // ! Debug
        // .fn_plugin(debug_plugin)
        // ! States
        .add_state::<GameState>()
        // ! Assets
        .fn_plugin(load_assets_plugin)
        // !Camera
        .fn_plugin(camera_plugin)
        // ! Spawn entities
        .add_system(spawn_ground.in_schedule(OnEnter(GameState::Run)))
        // ! Level
        .fn_plugin(map_plugin)
        .add_system(spawn_player.in_set(OnUpdate(GameState::Run)))
        .add_system(spawn_walls.in_set(OnUpdate(GameState::Run)))
        // ! Animation
        .add_system(animate_sprites)
        .add_system(update_direction)
        // ! Movement
        .add_system(move_player_system)
        // ! UI
        .run();
}
