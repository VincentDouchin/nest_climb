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
        // ! Camera
        .fn_plugin(camera_plugin)
        // ! Level
        .fn_plugin(parallax_plugin)
        .fn_plugin(map_plugin)
        .add_systems(
            (spawn_player, spawn_walls, spawn_enemy, spawn_collectibles)
                .in_set(OnUpdate(GameState::Run)),
        )
        // ! Collectibles
        .add_system(collect_collectible.in_set(OnUpdate(GameState::Run)))
        // ! Movement
        .add_system(
            move_player_system
                .before(move_camera)
                .in_set(OnUpdate(GameState::Run)),
        )
        .add_system(patrol.in_set(OnUpdate(GameState::Run)))
        // ! Damage
        .fn_plugin(run_timer_plugin)
        .add_systems(
            (player_enemy_interaction, kill_entity, detect_health_changed)
                .chain()
                .in_set(OnUpdate(GameState::Run)),
        )
        // ! Animation
        .fn_plugin(animation_plugin)
        // ! UI
        .fn_plugin(run_ui_plugin)
        .add_system(spawn_pause_ui.in_schedule(OnEnter(GameState::Pause)))
        // ! START
        .add_system(spawn_start_ui.in_schedule(OnEnter(GameState::Start)))
        .add_system(start_game.in_set(OnUpdate(GameState::Start)))
        // ! LEVEL SELECT
        .add_system(spawn_level_select_ui.in_schedule(OnEnter(GameState::LevelSelect)))
        .add_system(select_level.in_set(OnUpdate(GameState::LevelSelect)))
        // ! PAUSE
        .add_system(go_back_to_level_select.in_set(OnUpdate(GameState::Pause)))
        .run();
}
