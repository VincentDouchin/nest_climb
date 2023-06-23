use bevy::prelude::*;
use bevy_ui_navigation::prelude::Focusable;

use crate::*;

pub fn spawn_game_over_ui(mut commands: Commands, assets: Res<MyAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                    },
                    ..default()
                },
                ..default()
            },
            StateUi(PauseState::GameOver),
        ))
        .with_children(|root| {
            root.spawn(TextBundle {
                text: Text::from_section(
                    "Game Over",
                    TextStyle {
                        font: assets.default_font.clone(),
                        font_size: 50.0,
                        color: Color::RED,
                    },
                ),
                ..default()
            });
            root.spawn((
                ButtonBundle::default(),
                Focusable::default(),
                MenuButton::GoToGameState(GameState::LevelSelect),
            ))
            .with_children(|button| {
                button.spawn(TextBundle {
                    text: Text::from_section(
                        "Go to level select",
                        TextStyle {
                            font: assets.default_font.clone(),
                            font_size: 50.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                });
            });
            root.spawn((
                ButtonBundle::default(),
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
        });
}
