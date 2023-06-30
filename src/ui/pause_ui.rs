use crate::*;
use bevy::prelude::*;
use bevy_ui_navigation::prelude::Focusable;
pub fn spawn_pause_ui(mut commands: Commands, assets: Res<MyAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    padding: UiRect::all(Val::Px(100.0)),
                    gap: Size::all(Val::Px(20.0)),
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            },
            NineSlice {
                image_handle: assets.frame_big.clone(),
                margins: Vec4::splat(32.0),
                scale: 3.0,
                ..default()
            },
            StateUi(PauseState::Paused),
        ))
        .with_children(|root| {
            root.spawn(TextBundle {
                text: Text::from_section(
                    "Game paused",
                    TextStyle {
                        font: assets.default_font.clone(),
                        font_size: 50.0,
                        color: Color::BLACK,
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..default()
            });
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
            .iter()
            .for_each(|(text, menu_button)| {
                root.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Focusable::default(),
                    menu_button.to_owned(),
                    ButtonImages::new(&assets.frame_small, &assets.frame_small_selected),
                    NineSlice {
                        image_handle: assets.frame_small.clone(),
                        margins: Vec4::splat(8.0),
                        ..default()
                    },
                ))
                .with_children(|button| {
                    button.spawn(TextBundle {
                        style: Style {
                            size: Size::width(Val::Percent(100.0)),
                            ..default()
                        },
                        text: Text::from_section(
                            text.clone(),
                            TextStyle {
                                font: assets.default_font.clone(),
                                font_size: 30.0,
                                color: Color::BLACK,
                            },
                        )
                        .with_alignment(TextAlignment::Right),
                        ..default()
                    });
                });
            });
        });
}
