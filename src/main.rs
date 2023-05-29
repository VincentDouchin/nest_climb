use bevy::prelude::*;

use nest_climb::*;
use seldom_fn_plugin::FnPluginExt;

fn main() {
    App::new()
        // ! Libraries
        .fn_plugin(initialize_libraries)
        // ! Debug
        .fn_plugin(debug_plugin)
        // ! States
        .add_state::<GameState>()
        .fn_plugin(pause_plugin)
        .add_system(despawn_state_ui)
        // ! Assets
        .fn_plugin(load_assets_plugin)
        // !Camera
        .fn_plugin(camera_plugin)
        // ! Level
        .fn_plugin(parallax_plugin)
        .fn_plugin(map_plugin)
        .add_system(spawn_player.in_set(OnUpdate(GameState::Run)))
        .add_system(spawn_enemy.in_set(OnUpdate(GameState::Run)))
        .add_system(spawn_walls.in_set(OnUpdate(GameState::Run)))
        // ! Movement
        .add_system(
            move_player_system
                .before(move_camera)
                .in_set(OnUpdate(GameState::Run)),
        )
        // ! Animation
        .add_system(animate_sprites.in_set(OnUpdate(GameState::Run)))
        .add_system(update_direction.in_set(OnUpdate(GameState::Run)))
        // ! UI
        .add_system(spawn_run_ui.in_schedule(OnEnter(GameState::Run)))
        .add_system(spawn_pause_ui.in_schedule(OnEnter(GameState::Pause)))
        .add_system(display_hearts.in_set(OnUpdate(GameState::Run)))
        // ! START
        .add_system(spawn_start_ui.in_schedule(OnEnter(GameState::Start)))
        .add_system(start_game.in_set(OnUpdate(GameState::Start)))
        // ! LEVEL SELECT
        .add_system(spawn_level_select_ui.in_schedule(OnEnter(GameState::LevelSelect)))
        .add_system(select_level.in_set(OnUpdate(GameState::LevelSelect)))
        .run();
}
