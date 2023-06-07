use crate::*;
use bevy::{prelude::*, window::WindowFocused};
use bevy_rapier2d::prelude::*;
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Run,
    Start,
    LevelSelect,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum PauseState {
    #[default]
    NotPaused,
    Paused,
}

pub fn pause_game(
    keys: Res<Input<KeyCode>>,
    mut next_paused_state: ResMut<NextState<PauseState>>,
    current_paused_state: Res<State<PauseState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        let next_state = if current_paused_state.0 == PauseState::Paused {
            PauseState::NotPaused
        } else {
            PauseState::Paused
        };
        next_paused_state.set(next_state)
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
pub fn pause_physics<const PAUSE: bool>(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = PAUSE;
}

pub fn pause_plugin(app: &mut App) {
    app.add_state::<PauseState>()
        .add_system(pause_physics::<false>.in_schedule(OnEnter(PauseState::Paused)))
        .add_system(pause_physics::<true>.in_schedule(OnExit(PauseState::Paused)))
        .add_system(spawn_pause_ui.in_schedule(OnEnter(PauseState::Paused)))
        .add_system(pause_game)
        .add_system(pause_game_on_unfocus);
}
