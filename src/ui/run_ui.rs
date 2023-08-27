use crate::*;
use bevy::prelude::*;
use bevy_ui_navigation::prelude::*;
use leafwing_input_manager::{action_state::ActionStateDriverTarget, prelude::*};
#[derive(Component)]
pub struct HeartContainer;
#[derive(Resource, Default)]
pub struct IsTouchDevice(pub bool);

#[derive(Resource)]
pub struct Score {
    pub enemies_killed: u32,
    pub collectibles: u32,
}
impl Score {
    fn sum(&self) -> u32 {
        return &self.collectibles + &self.enemies_killed * 100;
    }
}
impl Default for Score {
    fn default() -> Self {
        Score {
            enemies_killed: 0,
            collectibles: 0,
        }
    }
}

#[derive(Component)]
pub struct ButtonImages {
    pub normal: Handle<Image>,
    pub pressed: Handle<Image>,
}
impl ButtonImages {
    pub fn new(normal: &Handle<Image>, pressed: &Handle<Image>) -> Self {
        ButtonImages {
            normal: normal.clone(),
            pressed: pressed.clone(),
        }
    }
}

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct PlayerButtons;

pub fn spawn_run_ui(mut commands: Commands, assets: Res<MyAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::SpaceBetween,
                    position_type: PositionType::Absolute,

                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    ..default()
                },
                ..default()
            },
            StateUi(GameState::Run),
        ))
        .with_children(|root| {
            root.spawn(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,

                    margin: UiRect::all(Val::Px(16.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|display| {
                display.spawn((
                    NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            padding: UiRect::all(Val::Px(16.0)),

                            ..default()
                        },
                        ..default()
                    },
                    NineSlice {
                        image_handle: assets.heart_container.clone(),
                        margins: Vec4::splat(8.0),
                        dynamic: true,
                        scale: 3.0,
                        layer: 2,
                        ..default()
                    },
                    HeartContainer,
                ));
                display
                    .spawn(ImageBundle {
                        image: UiImage::new(assets.coin_container.clone()),
                        style: Style {
                            width: Val::Px(41.0 * 3.0),
                            height: Val::Px(23.0 * 3.0),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|coin_container| {
                        coin_container.spawn((
                            TextBundle {
                                text: Text::from_section(
                                    0.to_string(),
                                    TextStyle {
                                        font: assets.default_font.clone(),
                                        font_size: 40.0,
                                        color: Color::BLACK,
                                    },
                                ),
                                style: Style {
                                    margin: UiRect {
                                        left: Val::Percent(35.0),
                                        bottom: Val::Auto,
                                        top: Val::Auto,
                                        right: Val::Auto,
                                    },
                                    ..default()
                                },
                                ..default()
                            },
                            ScoreDisplay,
                        ));
                    });
            });
        });
}

#[derive(Component)]
pub struct HeartUI {
    pub position: u32,
}

pub fn update_health_ui(
    mut commands: Commands,
    mut hearts_query: Query<(&mut UiImage, &HeartUI, Entity), With<HeartUI>>,
    hearts_container_query: Query<Entity, With<HeartContainer>>,
    player_query: Query<&Health, (With<Player>, Changed<Health>)>,
    assets: Res<MyAssets>,
) {
    if let Ok(container) = hearts_container_query.get_single() {
        if let Ok(player_health) = player_query.get_single() {
            // Check existing hearts
            for i in 0..player_health.max_health {
                let handle = if player_health.current_health <= i {
                    assets.heart_empty.clone()
                } else {
                    assets.heart_full.clone()
                };
                // Check image if heart already exists
                if let Some((mut ui_image, _, _)) = hearts_query
                    .iter_mut()
                    .find(|(_, heart, _)| heart.position == i)
                {
                    if ui_image.texture != handle {
                        ui_image.texture = handle
                    }
                } else {
                    // Create heart
                    let heart = commands
                        .spawn((
                            ImageBundle {
                                image: UiImage {
                                    texture: handle,
                                    ..default()
                                },
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                ..default()
                            },
                            HeartUI { position: i },
                        ))
                        .id();
                    commands.entity(container).add_child(heart);
                }
            }
            // delete extra hearts
            for (_, heart, entity) in hearts_query.iter() {
                if heart.position >= player_health.max_health {
                    commands.entity(entity).despawn_recursive()
                }
            }
        }
    }
}

pub fn update_score(
    mut score_display_query: Query<&mut Text, With<ScoreDisplay>>,
    score: Res<Score>,
) {
    for mut text in score_display_query.iter_mut() {
        text.sections[0].value = score.sum().to_string();
    }
}

pub fn reset_score(mut score: ResMut<Score>) {
    score.enemies_killed = 0;
}

pub fn spawn_touch_buttons(
    mut commands: Commands,
    assets: Res<MyAssets>,
    player_query: Query<Entity, Added<Player>>,
    menu_input_query: Query<Entity, With<ActionState<MenuAction>>>,
) {
    if let Ok(player_entity) = player_query.get_single() {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),

                        ..default()
                    },
                    ..default()
                },
                PlayerButtons,
                StateUi(GameState::Run),
            ))
            .with_children(|root| {
                if let Ok(menu_input) = menu_input_query.get_single() {
                    root.spawn((
                        // ! PAUSE
                        ButtonBundle {
                            image: UiImage::new(assets.button_pause.clone()),
                            style: Style {
                                position_type: PositionType::Absolute,
                                top: Val::Px(20.0),
                                right: Val::Px(20.0),
                                width: Val::Px(50.),
                                height: Val::Px(50.),
                                ..Default::default()
                            },

                            ..default()
                        },
                        ActionStateDriver {
                            action: MenuAction::Pause,
                            targets: ActionStateDriverTarget::Single(menu_input),
                        },
                        ButtonImages {
                            normal: assets.button_pause.clone(),
                            pressed: assets.button_pause_pressed.clone(),
                        },
                    ));
                }

                [
                    (
                        PlayerAction::MoveLeft,
                        assets.button_left.clone(),
                        assets.button_left_pressed.clone(),
                    ),
                    (
                        PlayerAction::Crouch,
                        assets.button_down.clone(),
                        assets.button_down_pressed.clone(),
                    ),
                    (
                        PlayerAction::MoveRight,
                        assets.button_right.clone(),
                        assets.button_right_pressed.clone(),
                    ),
                    (
                        PlayerAction::Jump,
                        assets.button_up.clone(),
                        assets.button_up_pressed.clone(),
                    ),
                ]
                .iter()
                .for_each(|(player_action, button, button_pressed)| {
                    root.spawn((
                        ButtonBundle {
                            image: UiImage::new(button.clone()),
                            style: Style {
                                align_self: AlignSelf::End,
                                margin: if player_action == &PlayerAction::Jump {
                                    UiRect {
                                        left: Val::Auto,
                                        right: Val::Px(20.0),
                                        top: Val::Px(20.0),
                                        bottom: Val::Px(20.0),
                                    }
                                } else {
                                    UiRect::all(Val::Px(20.0))
                                },
                                width: Val::Px(80.),
                                height: Val::Px(80.),
                                ..Default::default()
                            },

                            ..default()
                        },
                        ActionStateDriver {
                            action: *player_action,
                            targets: ActionStateDriverTarget::Single(player_entity),
                        },
                        ButtonImages {
                            normal: button.clone(),
                            pressed: button_pressed.clone(),
                        },
                    ));
                });
            });
    }
}

pub fn press_button(
    mut button_query: Query<(
        &ButtonImages,
        Option<&mut NineSlice>,
        Option<&Interaction>,
        Option<&Focusable>,
        &mut UiImage,
    )>,
) {
    for (button_images, maybe_nine_slice, interaction, focused, mut image_handle) in
        button_query.iter_mut()
    {
        let texture = if interaction.map_or(false, |inter| inter == &Interaction::Pressed)
            || focused.map_or(false, |focus| FocusState::Focused == focus.state())
        {
            button_images.pressed.clone()
        } else {
            button_images.normal.clone()
        };
        if let Some(mut nine_slice) = maybe_nine_slice {
            if nine_slice.image_handle != texture {
                nine_slice.image_handle = texture
            }
        } else if image_handle.texture != texture {
            image_handle.texture = texture
        }
    }
}

pub fn detect_touch(
    mut is_touch: ResMut<IsTouchDevice>,
    mut touch_events: EventReader<TouchInput>,
) {
    if touch_events.iter().len() > 0 {
        is_touch.0 = true
    }
}

pub fn multi_touch_button(
    mut button_query: Query<
        (&GlobalTransform, &mut Interaction, &Node),
        With<ActionStateDriver<PlayerAction>>,
    >,
    touches: Res<Touches>,
) {
    for (transform, mut interaction, node) in button_query.iter_mut() {
        let min_x = transform.translation().x - node.size().x / 2.0;
        let max_x = transform.translation().x + node.size().x / 2.0;
        let min_y = transform.translation().y - node.size().y / 2.0;
        let max_y = transform.translation().y + node.size().y / 2.0;
        *interaction = if touches.iter().any(|touch| {
            return (min_x..max_x).contains(&touch.position().x)
                && (min_y..max_y).contains(&touch.position().y);
        }) {
            Interaction::Pressed
        } else {
            Interaction::None
        };
    }
}

pub fn run_ui_plugin(app: &mut App) {
    use bevy::ui::ui_focus_system;
    app.init_resource::<Score>()
        .init_resource::<IsTouchDevice>()
        .add_systems(OnEnter(GameState::LevelSelect), reset_score)
        .add_systems(
            Update,
            (update_health_ui, update_score).run_if(in_state(GameState::Run)),
        );

    app.add_systems(
        Update,
        spawn_touch_buttons
            .run_if(in_state(GameState::Run))
            .run_if(|is_touch_device: Res<IsTouchDevice>| is_touch_device.0),
    )
    .add_systems(Update, multi_touch_button.after(ui_focus_system))
    .add_systems(
        Update,
        detect_touch.run_if(|is_touch_device: Res<IsTouchDevice>| !is_touch_device.0),
    )
    .add_systems(Update, press_button);
}
