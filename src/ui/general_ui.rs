use crate::*;
use bevy::prelude::*;
use bevy_ui_navigation::prelude::Focusable;
pub fn spawn_menu<T: Component + Clone, U: States>(
    mut commands: Commands,
    assets: Res<MyAssets>,
    title: &str,
    state_ui: StateUi<U>,
    mut buttons: Vec<(&str, T)>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    padding: UiRect::all(Val::Px(96.0)),
                    gap: Size::all(Val::Px(20.0)),
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                ..default()
            },
            NineSlice {
                image_handle: assets.frame_big.clone(),
                margins: Vec4::splat(32.0),
                scale: 3.0,
                ..default()
            },
            state_ui,
        ))
        .with_children(|root| {
            root.spawn(TextBundle {
                text: Text::from_section(
                    title,
                    TextStyle {
                        font: assets.default_font.clone(),
                        font_size: 50.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
            for (text, menu_button) in buttons.iter_mut() {
                root.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Focusable::default(),
                    menu_button.clone(),
                    ButtonImages::new(&assets.frame_small, &assets.frame_small_selected),
                    NineSlice {
                        image_handle: assets.frame_small.clone(),
                        margins: Vec4::splat(8.0),
                        ..default()
                    },
                ))
                .with_children(|button| {
                    button.spawn(TextBundle {
                        text: Text::from_section(
                            text.clone(),
                            TextStyle {
                                font: assets.default_font.clone(),
                                font_size: 30.0,
                                color: Color::BLACK,
                            },
                        ),

                        ..default()
                    });
                });
            }
        });
}
