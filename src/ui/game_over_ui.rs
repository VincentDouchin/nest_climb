use crate::*;
use bevy::prelude::*;

pub fn spawn_game_over_ui(commands: Commands, assets: Res<MyAssets>) {
    spawn_menu(
        commands,
        assets,
        "Game Over",
        StateUi(PauseState::GameOver),
        [
            ("Retry", MenuButton::Retry),
            (
                "Choose level",
                MenuButton::GoToGameState(GameState::LevelSelect),
            ),
        ]
        .to_vec(),
    )
}
