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
            ("Retry", MenuButton::Retry),
            (
                "Choose level",
                MenuButton::GoToGameState(GameState::LevelSelect),
            ),
        ]
        .to_vec(),
    )
}
