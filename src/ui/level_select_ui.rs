use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ui_navigation::prelude::*;
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
                    padding: UiRect::all(Val::Percent(3.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|root| {
            // ! TOP TEXT

            root.spawn((
                NodeBundle {
                    style: Style {
                        size: Size {
                            height: Val::Percent(10.0),
                            ..default()
                        },
                        padding: UiRect::all(Val::Px(64.0)),
                        ..default()
                    },
                    ..default()
                },
                NineSlice {
                    image_handle: assets.frame_big.clone(),
                    margins: Vec4::splat(32.0),
                    scale: 1.0,
                    layer: 2,
                    ..default()
                },
            ))
            .with_children(|toptext| {
                toptext.spawn(TextBundle {
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

            root.spawn((
                NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(50.0)),
                        gap: Size::all(Val::Px(50.0)),
                        flex_grow: 1.0,
                        ..default()
                    },
                    ..default()
                },
                NineSlice {
                    image_handle: assets.frame_big.clone(),
                    margins: Vec4::splat(32.0),
                    scale: 1.0,
                    layer: 2,
                    ..default()
                },
            ))
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
                                        padding: UiRect::all(Val::Px(32.0)),
                                        ..default()
                                    },

                                    ..default()
                                },
                                NineSlice {
                                    image_handle: assets.button_big.clone(),
                                    margins: Vec4::splat(16.0),
                                    ..default()
                                },
                                ButtonImages::new(&assets.button_big, &assets.button_big_pressed),
                                Focusable::default(),
                                MenuButton::LevelSelect {
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
