use bevy::prelude::*;

use nest_climb::*;
use seldom_fn_plugin::FnPluginExt;

fn main() {
    App::new()
        // ! Libraries
        .fn_plugin(initialize_libraries)
        // ! States
        .add_state::<GameState>()
        .fn_plugin(pause_plugin)
        .add_system(despawn_state_ui::<GameState>)
        .add_system(despawn_state_ui::<PauseState>)
        // ! Assets
        .fn_plugin(load_assets_plugin)
        // ! Camera
        .fn_plugin(camera_plugin)
        // ! Level
        .fn_plugin(parallax_plugin)
        .fn_plugin(map_plugin)
        // ! RUN
        .add_systems(
            (
                collect_collectible,
                patrol,
                player_enemy_interaction,
                kill_entity,
                detect_health_changed,
                jump_throught_platforms,
            )
                .in_set(OnUpdate(GameState::Run))
                .distributive_run_if(in_state(PauseState::NotPaused)),
        )
        // ! Movement
        .add_system(
            move_player_system
                .before(move_camera)
                .in_set(OnUpdate(GameState::Run)),
        )
        // ! Damage
        .fn_plugin(run_timer_plugin)
        // ! Animation
        .fn_plugin(animation_plugin)
        // ! UI
        .fn_plugin(run_ui_plugin)
        // ! START
        .add_system(spawn_start_ui.in_schedule(OnEnter(GameState::Start)))
        .add_system(start_game.in_set(OnUpdate(GameState::Start)))
        // ! LEVEL SELECT
        .add_system(spawn_level_select_ui.in_schedule(OnEnter(GameState::LevelSelect)))
        .add_system(select_level.in_set(OnUpdate(GameState::LevelSelect)))
        // ! PAUSE
        .add_system(go_back_to_level_select)
        // ! FLAG
        .add_system(spawn_flag.in_set(OnUpdate(GameState::Run)))
        .add_system(level_transition.in_schedule(OnEnter(GameState::LevelTransition)))
        .add_system(move_to_next_level.in_set(OnUpdate(GameState::Run)))
        // ! Debug
        .fn_plugin(debug_plugin)
        .run();
}
