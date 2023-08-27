use crate::*;
use bevy::{prelude::*, window::WindowFocused};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::ActionState;
pub fn pause_game(
    menu_inputs: Query<&ActionState<MenuAction>>,
    mut next_paused_state: ResMut<NextState<PauseState>>,
    current_paused_state: Res<State<PauseState>>,
) {
    for input in menu_inputs.iter() {
        if input.just_pressed(MenuAction::UnPause)
            && current_paused_state.get() == &PauseState::Paused
        {
            next_paused_state.set(PauseState::NotPaused)
        }
        if input.just_pressed(MenuAction::Pause)
            && current_paused_state.get() == &PauseState::NotPaused
        {
            next_paused_state.set(PauseState::Paused)
        }
    }
}

pub fn pause_game_on_unfocus(
    mut events: EventReader<WindowFocused>,
    mut paused_state: ResMut<NextState<PauseState>>,
    current_paused_state: Res<State<PauseState>>,
) {
    if events.iter().any(|event| !event.focused)
        && current_paused_state.get() != &PauseState::Paused
    {
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
        .add_systems(OnExit(PauseState::NotPaused), pause_physics::<false>)
        .add_systems(OnEnter(PauseState::NotPaused), pause_physics::<true>)
        .add_systems(OnExit(GameState::Run), unpause_game)
        .add_systems(
            Update,
            (pause_game, pause_game_on_unfocus).run_if(in_state(GameState::Run)),
        );
}
