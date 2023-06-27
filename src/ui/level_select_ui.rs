use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ninepatch::*;
use bevy_ui_navigation::prelude::*;
pub fn spawn_level_select_ui(
    mut commands: Commands,
    assets: Res<MyAssets>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
    ldtk: Res<Assets<LdtkAsset>>,
) {
    let nine_patch_48 = nine_patches.add(NinePatchBuilder::by_margins(32, 32, 32, 32));
    commands
        // ! ROOT
        .spawn((
            StateUi(GameState::LevelSelect),
            NodeBundle {
                style: Style {
                    size: Size::all(Val::Percent(100.0)),
                    padding: UiRect::all(Val::Px(50.0)),
                    gap: Size::all(Val::Px(50.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        // ! TOP TEXT
        .with_children(|root| {
            let text = root
                .spawn(TextBundle {
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
                })
                .id();
            root.spawn(NinePatchBundle {
                style: Style {
                    size: Size {
                        height: Val::Percent(10.0),
                        ..default()
                    },
                    ..default()
                },
                nine_patch_data: NinePatchData::with_single_content(
                    assets.frame_big.clone(),
                    nine_patch_48.clone(),
                    text,
                ),
                ..default()
            });
            // ! LEVEL CONTAINER
            let levels = root
                .spawn(NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(50.0)),
                        gap: Size::all(Val::Px(50.0)),
                        ..default()
                    },
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

                                        ..default()
                                    },
                                    NineSlice {
                                        image_handle: assets.frame_small.clone(),
                                        margins: Vec4::splat(8.0),
                                    },
                                    ButtonImages::new(
                                        &assets.frame_small,
                                        &assets.frame_small_selected,
                                    ),
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
                })
                .id();
            root.spawn(NinePatchBundle {
                style: Style {
                    flex_grow: 1.0,
                    ..default()
                },
                nine_patch_data: NinePatchData::with_single_content(
                    assets.frame_big.clone(),
                    nine_patch_48.clone(),
                    levels,
                ),
                ..default()
            });
        });
}
