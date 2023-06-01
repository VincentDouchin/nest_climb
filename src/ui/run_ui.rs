use crate::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct HeartContainer;

#[derive(Resource)]
pub struct Score {
    pub enemies_killed: u32,
}
impl Default for Score {
    fn default() -> Self {
        Score { enemies_killed: 0 }
    }
}

#[derive(Component)]
pub struct ScoreDisplay;

pub fn spawn_run_ui(mut commands: Commands, assets: Res<MyAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::SpaceBetween,
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Auto,
                    },
                    ..default()
                },
                ..default()
            },
            StateUi {
                state: GameState::Run,
            },
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
        text.sections[0].value = score.enemies_killed.to_string();
    }
}

pub fn reset_score(mut score: ResMut<Score>) {
    score.enemies_killed = 0;
}

pub fn run_ui_plugin(app: &mut App) {
    app.init_resource::<Score>()
        .add_system(reset_score.in_schedule(OnEnter(GameState::LevelSelect)))
        .add_system(spawn_run_ui.in_schedule(OnEnter(GameState::Run)))
        .add_systems((update_health_ui, update_score).in_set(OnUpdate(GameState::Run)));
}
