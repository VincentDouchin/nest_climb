use crate::*;
use bevy::prelude::*;
pub fn spawn_pause_ui(commands: Commands, assets: Res<MyAssets>) {
    spawn_menu(
        commands,
        assets,
        "Game paused",
        StateUi(PauseState::Paused),
        [
            ("Resume", MenuButton::GoToPausedState(PauseState::NotPaused)),
            (
                "Retry",
                MenuButton::GoToGameState(GameState::LevelTransition),
            ),
            (
                "Choose level",
                MenuButton::GoToGameState(GameState::LevelSelect),
            ),
        ]
        .to_vec(),
    )
}
