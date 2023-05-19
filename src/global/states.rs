use bevy::{prelude::*, window::WindowFocused};
use bevy_rapier2d::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    InitRun,
    Run,
    Pause,
}

pub fn switch_state(
    app_state: Res<State<GameState>>,
    keys: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if app_state.0 == GameState::Run {
            next_state.0 = Some(GameState::Pause)
        }
        if app_state.0 == GameState::Pause {
            next_state.set(GameState::Run)
        }
    }
}

pub fn pause_game(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = false;
}
pub fn resume_game(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = true;
}
pub fn pause_on_focus_lost(
    mut events: EventReader<WindowFocused>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in events.iter() {
        if !event.focused {
            next_state.set(GameState::Pause)
        }
    }
}
pub fn pause_plugin(app: &mut App) {
    app.add_system(switch_state)
        .add_system(pause_game.in_schedule(OnEnter(GameState::Pause)))
        .add_system(resume_game.in_schedule(OnExit(GameState::Pause)))
        .add_system(pause_on_focus_lost.in_set(OnUpdate(GameState::Run)));
}
