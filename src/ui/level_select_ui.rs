use crate::{CurrentLevel, GameState, MyAssets, StateUi};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component)]
pub struct LevelSelector {
    file: Handle<LdtkAsset>,
    level: usize,
}

pub fn spawn_level_select_ui(
    mut commands: Commands,
    assets: Res<MyAssets>,
    ldtk: Res<Assets<LdtkAsset>>,
) {
    commands
        // ! ROOT
        .spawn((
            StateUi(GameState::LevelSelect),
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
                    gap: Size::all(Val::Px(50.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            })
            .with_children(|level_container| {
                // ! LEVEL BUTTON
                if let Some(ldtk_file) = ldtk.get(&assets.test_level) {
                    let levels = ldtk_file.iter_levels();
                    for (index, _) in levels.enumerate() {
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
                                LevelSelector {
                                    file: assets.test_level.clone(),
                                    level: index,
                                },
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
                }
            });
        });
}

pub fn select_level(
    mut commands: Commands,
    level_button_query: Query<(&LevelSelector, &Interaction), With<Button>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (level_selector, interaction) in level_button_query.iter() {
        if interaction == &Interaction::Clicked {
            commands.insert_resource(CurrentLevel {
                file: level_selector.file.clone(),
            });
            commands.insert_resource(LevelSelection::Index(level_selector.level));
            next_state.set(GameState::Run);
        }
    }
}
