use bevy::prelude::*;
use bevy_tnua::*;
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
        .add_startup_system(spawn_inputs)
        // ! States
        .fn_plugin(pause_plugin)
        .add_system(despawn_state_ui::<GameState>)
        .add_system(despawn_state_ui::<PauseState>)
        // ! Background
        .fn_plugin(background_plugin)
        // ! Camera
        .fn_plugin(camera_plugin)
        // ! Level
        .fn_plugin(map_plugin)
        // ! RUN
        .add_systems(
            (
                collect_item,
                patrol,
                move_pendulum,
                move_ghost,
                player_enemy_interaction,
                kill_entity,
                detect_health_changed,
                // jump_throught_platforms,
                // bounce_on_trampoline.before(move_player_system),
            )
                .in_set(OnUpdate(GameState::Run))
                .distributive_run_if(in_state(PauseState::NotPaused)),
        )
        // ! Movement
        .fn_plugin(movement_plugin)
        .add_system(
            move_player_system
                .before(apply_movement)
                .run_if(not(in_state(PauseState::Paused))),
        )
        // ! Damage
        .fn_plugin(run_timer_plugin)
        // ! Animation
        .fn_plugin(animation_plugin)
        // ! NAVIGATION
        .add_system(click_on_buttons)
        // ! START
        // .add_system(start_game.in_set(OnUpdate(GameState::Start)))
        // ! LEVEL SELECT
        // .add_system(select_level.in_set(:OnUpdate(GameState::LevelSelect)))
        // ! NEST
        .add_system(level_transition.in_schedule(OnEnter(GameState::LevelTransition)))
        .add_system(move_to_next_level.in_set(OnUpdate(GameState::Run)))
        // ! CLIMBING
        // .add_systems((ignore_gravity_if_climbing, detect_can_climb))
        // ! UI
        .fn_plugin(selector_plugin)
        .fn_plugin(run_ui_plugin)
        .add_system(spawn_run_ui.in_schedule(OnEnter(GameState::Run)))
        .add_system(spawn_start_ui.in_schedule(OnEnter(GameState::Start)))
        .add_system(move_clouds)
        .add_system(spawn_level_select_ui.in_schedule(OnEnter(GameState::LevelSelect)))
        .add_system(spawn_pause_ui.in_schedule(OnEnter(PauseState::Paused)))
        .add_system(spawn_game_over_ui.in_schedule(OnEnter(PauseState::GameOver)))
        // ! Parallax
        .add_system(spawn_parallax.in_schedule(OnEnter(GameState::Run)))
        .add_system(move_parallax.in_set(OnUpdate(GameState::Run)))
        // ! Debug
        .fn_plugin(debug_plugin)
        .run();
}
