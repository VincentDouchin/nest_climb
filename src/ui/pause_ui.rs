use crate::*;
use bevy::prelude::*;
use bevy_ui_navigation::prelude::Focusable;

pub fn spawn_pause_ui(mut commands: Commands, assets: Res<MyAssets>) {
    // ! ROOT
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    margin: UiRect::all(Val::Auto),
                    size: Size {
                        width: Val::Percent(30.0),
                        height: Val::Percent(50.0),
                    },

                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            },
            StateUi(PauseState::Paused),
        ))
        .with_children(|root| {
            // ! GAME PAUSED TEXT
            root.spawn(TextBundle {
                style: Style { ..default() },

                text: Text::from_section(
                    "Game paused",
                    TextStyle {
                        font: assets.default_font.clone(),
                        font_size: 50.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
            // ! Resume
            root.spawn((
                ButtonBundle { ..default() },
                Focusable::default(),
                MenuButton::GoToPausedState(PauseState::NotPaused),
            ))
            .with_children(|button| {
                button.spawn(TextBundle {
                    text: Text::from_section(
                        "Resume",
                        TextStyle {
                            font: assets.default_font.clone(),
                            font_size: 50.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                });
            });
            // ! Retry
            root.spawn((
                ButtonBundle { ..default() },
                Focusable::default(),
                MenuButton::GoToGameState(GameState::LevelTransition),
            ))
            .with_children(|button| {
                button.spawn(TextBundle {
                    text: Text::from_section(
                        "Retry",
                        TextStyle {
                            font: assets.default_font.clone(),
                            font_size: 50.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                });
            });
            // ! LEVEL SELECT BUTTON
            root.spawn((
                ButtonBundle { ..default() },
                Focusable::default(),
                MenuButton::GoToGameState(GameState::LevelSelect),
            ))
            .with_children(|button| {
                button.spawn(TextBundle {
                    text: Text::from_section(
                        "Go back to menu",
                        TextStyle {
                            font: assets.default_font.clone(),
                            font_size: 50.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                });
            });
        });
}
