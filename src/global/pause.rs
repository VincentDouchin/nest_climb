use crate::*;
use bevy::{prelude::*, window::WindowFocused};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::ActionState;
pub fn pause_game(
    menu_inputs: Query<&ActionState<PlayerAction>>,
    mut next_paused_state: ResMut<NextState<PauseState>>,
    current_paused_state: Res<State<PauseState>>,
) {
    for input in menu_inputs.iter() {
        if input.just_pressed(PlayerAction::Pause) {
            if current_paused_state.0 == PauseState::Paused {
                next_paused_state.set(PauseState::NotPaused)
            } else if current_paused_state.0 == PauseState::NotPaused {
                next_paused_state.set(PauseState::Paused)
            };
        }
    }
}

pub fn pause_game_on_unfocus(
    mut events: EventReader<WindowFocused>,
    mut paused_state: ResMut<NextState<PauseState>>,
) {
    if events.iter().any(|event| !event.focused) {
        paused_state.set(PauseState::Paused)
    }
}
pub fn unpause_game(mut paused_state: ResMut<NextState<PauseState>>) {
    paused_state.set(PauseState::NotPaused)
}
pub fn pause_physics<const PAUSE: bool>(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = PAUSE;
}

pub fn pause_plugin(app: &mut App) {
    app.add_state::<PauseState>()
        .add_system(pause_physics::<false>.in_schedule(OnExit(PauseState::NotPaused)))
        .add_system(pause_physics::<true>.in_schedule(OnEnter(PauseState::NotPaused)))
        .add_system(unpause_game.in_schedule(OnExit(GameState::Run)))
        .add_systems((pause_game, pause_game_on_unfocus).in_set(OnUpdate(GameState::Run)));
}
