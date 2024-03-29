use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ui_navigation::prelude::*;

#[derive(Component, Clone)]
pub enum MenuButton {
    LevelSelect {
        file: Handle<LdtkAsset>,
        level: usize,
    },
    GoToGameState(GameState),
    GoToPausedState(PauseState),
    Retry,
}

pub fn click_on_buttons(
    mut buttons: Query<&mut MenuButton>,
    mut events: EventReader<NavEvent>,
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
    mut next_transition_state: ResMut<NextState<TransitionState>>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut buttons,
        |mut button| match &mut *button {
            MenuButton::LevelSelect { file, level } => {
                commands.insert_resource(CurrentLevel {
                    file: Some(file.clone()),
                });
                commands.insert_resource(LevelSelection::Index(*level));
                next_transition_state.set(TransitionState::In);
            }
            MenuButton::GoToGameState(state) => next_game_state.set(state.clone()),
            MenuButton::GoToPausedState(state) => next_pause_state.set(state.clone()),
            MenuButton::Retry => {
                next_transition_state.set(TransitionState::In);
                next_pause_state.set(PauseState::NotPaused)
            }
        },
    )
}
