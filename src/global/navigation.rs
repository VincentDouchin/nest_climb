use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ui_navigation::prelude::*;
pub fn button_system(
    mut interaction_query: Query<(&Focusable, &mut BackgroundColor), Changed<Focusable>>,
) {
    for (focusable, mut material) in interaction_query.iter_mut() {
        if let FocusState::Focused = focusable.state() {
            *material = Color::ORANGE_RED.into();
        } else {
            *material = Color::DARK_GRAY.into();
        }
    }
}

#[derive(Component, Clone)]
pub enum MenuButton {
    LevelSelect {
        file: Handle<LdtkAsset>,
        level: usize,
    },
    GoToGameState(GameState),
    GoToPausedState(PauseState),
}

pub fn click_on_buttons(
    mut buttons: Query<&mut MenuButton>,
    mut events: EventReader<NavEvent>,
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_pause_state: ResMut<NextState<PauseState>>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut buttons,
        |mut button| match &mut *button {
            MenuButton::LevelSelect { file, level } => {
                commands.insert_resource(CurrentLevel {
                    file: Some(file.clone()),
                });
                commands.insert_resource(LevelSelection::Index(*level));
                next_game_state.set(GameState::Run);
            }
            MenuButton::GoToGameState(state) => next_game_state.set(state.clone()),
            MenuButton::GoToPausedState(state) => next_pause_state.set(state.clone()),
        },
    )
}
