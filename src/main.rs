use bevy::prelude::*;

use nest_climb::*;
use seldom_fn_plugin::FnPluginExt;

fn main() {
    App::new()
        .add_state::<GameState>()
        // ! Assets
        .fn_plugin(load_assets_plugin)
        // ! Libraries
        .fn_plugin(initialize_libraries)
        .fn_plugin(nine_slice_plugin)
        .add_systems(Startup, spawn_inputs)
        // ! States
        .fn_plugin(pause_plugin)
        .add_systems(Update, despawn_state_ui::<GameState>)
        .add_systems(Update, despawn_state_ui::<PauseState>)
        // ! Background
        .fn_plugin(background_plugin)
        // ! Camera
        .fn_plugin(camera_plugin)
        // ! Level
        .fn_plugin(map_plugin)
        // ! RUN
        .add_systems(
            Update,
            (
                collect_item,
                patrol,
                move_pendulum,
                move_ghost,
                player_enemy_interaction,
                kill_entity,
                detect_health_changed,
                jump_through_platforms,
                bounce_on_trampoline,
                move_player_system,
            )
                .run_if(in_state(PauseState::NotPaused))
                .run_if(in_state(GameState::Run))
                .run_if(in_state(TransitionState::None)),
        )
        // ! Damage
        .fn_plugin(run_timer_plugin)
        // ! Animation
        .fn_plugin(animation_plugin)
        // ! NAVIGATION
        .add_systems(Update, click_on_buttons)
        // ! NEST
        .add_systems(Update, move_to_next_level.run_if(in_state(GameState::Run)))
        // ! CLIMBING
        .add_systems(Update, (ignore_gravity_if_climbing, detect_can_climb))
        // ! UI
        .fn_plugin(selector_plugin)
        .fn_plugin(run_ui_plugin)
        .add_systems(OnEnter(GameState::Run), spawn_run_ui)
        .add_systems(OnEnter(GameState::Start), spawn_start_ui)
        .add_systems(Update, move_clouds)
        .add_systems(OnEnter(GameState::LevelSelect), spawn_level_select_ui)
        .add_systems(OnEnter(PauseState::Paused), spawn_pause_ui)
        .add_systems(OnEnter(PauseState::GameOver), spawn_game_over_ui)
        // ! Parallax
        .add_systems(OnEnter(GameState::Run), spawn_parallax)
        .add_systems(Update, (move_parallax).run_if(in_state(GameState::Run)))
        // ! Debug
        .fn_plugin(debug_plugin)
        // ! Transitions
        .fn_plugin(transition_plugin)
        .run();
}
