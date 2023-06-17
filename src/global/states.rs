use bevy::prelude::*;
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    Run,
    Start,
    LevelSelect,
    LevelTransition,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum PauseState {
    #[default]
    NotPaused,
    Paused,
    GameOver,
}
