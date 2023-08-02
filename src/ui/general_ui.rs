use std::time::Duration;

use crate::*;
use bevy::prelude::{Vec2, *};
use bevy_easings::{Ease, EaseFunction};
use bevy_ui_navigation::prelude::*;
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
                    // gap: UiRect::all(Val::Px(20.0)),
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
                            padding: UiRect::all(Val::Px(16.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Focusable::default(),
                    menu_button.clone(),
                    ButtonImages::new(&assets.button_big, &assets.button_big_pressed),
                    NineSlice {
                        image_handle: assets.button_big.clone(),
                        margins: Vec4::splat(16.0),
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

fn get_selector_components(node: &Node, transform: &GlobalTransform, offset: f32) -> (Vec2, Vec2) {
    let size = node.size() + offset;
    let translation = transform.translation();
    let position = Vec2::new(translation.x - size.x / 2.0, translation.y - size.y / 2.0);

    return (position, size);
}

trait CompareToNode {
    fn to_ui_rect(self) -> UiRect;
}
impl CompareToNode for Vec2 {
    fn to_ui_rect(self) -> UiRect {
        return UiRect {
            left: Val::Px(self.x),
            top: Val::Px(self.y),
            ..default()
        };
    }
}

#[derive(Component)]
pub struct Selector {
    pub target_size: Vec2,
    pub target_position: Vec2,
}
impl Selector {
    fn is_different(&self, size: &Vec2, position: &Vec2) -> bool {
        return size != &self.target_size || position != &self.target_position;
    }
    fn new(size: Vec2, position: Vec2) -> Self {
        Selector {
            target_position: position,
            target_size: size,
        }
    }
}

pub fn focus_selector(
    focus_query: Query<(&Node, &GlobalTransform), (With<Focused>, Without<Selector>)>,
    mut selector_query: Query<(Entity, &mut Selector, &Style), With<Selector>>,
    mut commands: Commands,
    assets: Res<MyAssets>,
) {
    let focus = focus_query.get_single().ok();
    let selector = selector_query.get_single_mut().ok();
    if let Some((selector_entity, _, _)) = selector {
        if focus.is_none() {
            commands.entity(selector_entity).despawn_recursive();
        }
    }

    if let Some((focus_node, focus_transform)) = focus {
        let (focus_position, focus_size) =
            get_selector_components(focus_node, focus_transform, 16.0);

        if let Some((selector_entity, mut selector, selector_style)) = selector {
            if selector.is_different(&focus_size, &focus_position) {
                selector.target_size = focus_size;
                selector.target_position = focus_position;
                commands
                    .entity(selector_entity)
                    .insert(selector_style.clone().ease_to(
                        Style {
                            position_type: PositionType::Absolute,
                            left: focus_position.to_ui_rect().left,
                            right: focus_position.to_ui_rect().right,
                            top: focus_position.to_ui_rect().top,
                            bottom: focus_position.to_ui_rect().bottom,
                            width: Val::Px(focus_size.x),
                            height: Val::Px(focus_size.y),
                            ..default()
                        },
                        EaseFunction::CubicInOut,
                        bevy_easings::EasingType::Once {
                            duration: Duration::from_millis(500),
                        },
                    ));
            }
        } else {
            commands.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: focus_position.to_ui_rect().left,
                        right: focus_position.to_ui_rect().right,
                        top: focus_position.to_ui_rect().top,
                        bottom: focus_position.to_ui_rect().bottom,
                        width: Val::Px(focus_size.x),
                        height: Val::Px(focus_size.y),
                        ..default()
                    },
                    z_index: ZIndex::Global(32),
                    ..default()
                },
                NineSlice {
                    image_handle: assets.selector.clone(),
                    margins: Vec4::splat(16.0),
                    layer: 3,
                    scale: 2.0,
                },
                Selector::new(focus_size, focus_position),
            ));
        }
    }
}

pub fn selector_plugin(app: &mut App) {
    app.add_systems(
        Update,
        focus_selector.run_if(not(in_state(GameState::AssetLoading))),
    );
}
