use crate::{CurrentLevel, GameState, MyAssets, StateUi};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub fn spawn_level_select_ui(mut commands: Commands, assets: Res<MyAssets>) {
    commands
        // ! ROOT
        .spawn((
            StateUi {
                state: GameState::LevelSelect,
            },
            NodeBundle {
                style: Style {
                    size: Size::all(Val::Percent(100.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        // ! TOP TEXT
        .with_children(|root| {
            root.spawn(NodeBundle {
                style: Style {
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Percent(10.0),
                    },
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Select a level",
                        TextStyle {
                            font: assets.default_font.clone(),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    ),
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        ..default()
                    },
                    ..default()
                });
            });
            // ! LEVEL CONTAINER
            root.spawn(NodeBundle {
                style: Style {
                    flex_grow: 1.0,
                    margin: UiRect::all(Val::Px(50.0)),
                    padding: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            })
            .with_children(|level_container| {
                // ! LEVEL BUTTON
                let levels = vec![assets.test_level.clone()];
                for (index, level) in levels.iter().enumerate() {
                    level_container
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::all(Val::Px(50.0)),
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::GRAY),
                                ..default()
                            },
                            level.clone(),
                        ))
                        .with_children(|level_button| {
                            level_button.spawn(TextBundle {
                                text: Text::from_section(
                                    index.to_string(),
                                    TextStyle {
                                        font: assets.default_font.clone(),
                                        font_size: 30.0,
                                        color: Color::BLACK,
                                    },
                                ),
                                style: Style {
                                    margin: UiRect::all(Val::Auto),
                                    ..default()
                                },
                                ..default()
                            });
                        });
                }
            });
        });
}

pub fn select_level(
    mut commands: Commands,
    level_button_query: Query<(&Handle<LdtkAsset>, &Interaction), With<Button>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (level_handle, interaction) in level_button_query.iter() {
        if interaction == &Interaction::Clicked {
            commands.insert_resource(CurrentLevel {
                level: level_handle.clone(),
            });
            next_state.set(GameState::Run);
        }
    }
}
