use crate::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionStateDriver;
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
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Auto,
                    },
                    ..default()
                },
                ..default()
            },
            StateUi(GameState::Run),
        ))
        .with_children(|root| {
            root.spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        margin: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                },
                HeartContainer,
            ));
            root.spawn((
                TextBundle {
                    text: Text::from_section(
                        0.to_string(),
                        TextStyle {
                            font: assets.default_font.clone(),
                            font_size: 50.0,
                            color: Color::BLACK,
                        },
                    ),
                    ..default()
                },
                ScoreDisplay,
            ));
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
                                    size: Size::all(Val::Px(50.0)),
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
) {
    if let Ok(player_entity) = player_query.get_single() {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        align_items: AlignItems::End,
                        ..default()
                    },
                    ..default()
                },
                PlayerButtons,
            ))
            .with_children(|root| {
                [
                    PlayerAction::MoveLeft,
                    PlayerAction::MoveRight,
                    PlayerAction::Jump,
                ]
                .iter()
                .for_each(|player_action| {
                    root.spawn((
                        ButtonBundle {
                            image: UiImage::new(assets.button_normal.clone()),
                            style: Style {
                                margin: if player_action == &PlayerAction::Jump {
                                    UiRect {
                                        left: Val::Auto,
                                        right: Val::Px(50.0),
                                        top: Val::Px(50.0),
                                        bottom: Val::Px(50.0),
                                    }
                                } else {
                                    UiRect::all(Val::Px(50.0))
                                },
                                size: Size::new(Val::Px(50.), Val::Px(50.)),
                                ..Default::default()
                            },

                            ..default()
                        },
                        ActionStateDriver {
                            action: *player_action,
                            entity: player_entity,
                        },
                        ButtonImages {
                            normal: assets.button_normal.clone(),
                            pressed: assets.button_pressed.clone(),
                        },
                    ));
                });
            });
    }
}

pub fn despawn_player_buttons(
    mut commands: Commands,
    removed_player: RemovedComponents<Player>,
    player_buttons_query: Query<Entity, With<PlayerButtons>>,
) {
    if !removed_player.is_empty() {
        for entity in player_buttons_query.iter() {
            commands.entity(entity).despawn_recursive()
        }
    }
}
pub fn press_button(mut button_query: Query<(&ButtonImages, &Interaction, &mut UiImage)>) {
    for (button_images, interaction, mut image_handle) in button_query.iter_mut() {
        let texture = if interaction == &Interaction::Clicked {
            button_images.pressed.clone()
        } else {
            button_images.normal.clone()
        };
        if image_handle.texture != texture {
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

pub fn run_ui_plugin(app: &mut App) {
    app.init_resource::<Score>()
        .init_resource::<IsTouchDevice>()
        .add_system(reset_score.in_schedule(OnEnter(GameState::LevelSelect)))
        .add_system(spawn_run_ui.in_schedule(OnEnter(GameState::Run)))
        .add_systems((update_health_ui, update_score).in_set(OnUpdate(GameState::Run)));
    app.add_system(
        spawn_touch_buttons
            .in_set(OnUpdate(GameState::Run))
            .run_if(|is_touch_device: Res<IsTouchDevice>| is_touch_device.0),
    )
    .add_system(detect_touch)
    .add_systems((press_button, despawn_player_buttons));
}
