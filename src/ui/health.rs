use bevy::prelude::*;

use crate::*;

#[derive(Component)]
pub struct HeartUI {
    pub position: u32,
}

pub fn display_hearts(
    mut commands: Commands,
    mut hearts_query: Query<(&mut UiImage, &HeartUI, Entity), With<HeartUI>>,
    hearts_container_query: Query<Entity, With<HeartContainer>>,
    player_query: Query<&Health, With<Player>>,
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
