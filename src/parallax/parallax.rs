use bevy_pixel_camera::PixelProjection;

use crate::*;

#[derive(Component)]
pub struct Parallax(pub f32);

pub fn spawn_parallax(
    mut commands: Commands,
    assets: Res<MyAssets>,
    images: ResMut<Assets<Image>>,
    camera_query: Query<&PixelProjection, With<MainCamera>>,
) {
    if let Ok(projection) = camera_query.get_single() {
        commands
            .spawn((
                TransformBundle::default(),
                FollowCamera {
                    offset: Vec2::new(0.0, -0.5),
                },
                StateUi(GameState::Run),
                VisibilityBundle::default(),
            ))
            .with_children(|origin| {
                for (index, (tree_handle, color_hex)) in [
                    (assets.tree_1.clone(), Color::hex("7cacda")),
                    (assets.tree_2.clone(), Color::hex("5c8fc8")),
                    (assets.tree_3.clone(), Color::hex("4c7dac")),
                    (assets.tree_4.clone(), Color::hex("497198")),
                ]
                .iter()
                .enumerate()
                {
                    let color = color_hex.as_ref().ok().unwrap();
                    let v_offset = (4.0 - index as f32) * 16.0;
                    let image = images.get(tree_handle).unwrap();
                    let direction = if index % 2 == 0 { 1.0 } else { -1.0 };
                    origin
                        .spawn((
                            SpriteBundle {
                                texture: tree_handle.clone(),
                                sprite: Sprite {
                                    color: color.clone(),
                                    ..default()
                                },
                                transform: Transform::from_translation(Vec3::new(
                                    index as f32 * 70.0 * direction,
                                    image.size().y / 2.0 + v_offset,
                                    (index as f32 * 0.01) + 0.1,
                                )),
                                ..default()
                            },
                            Parallax(index as f32 * 0.05),
                        ))
                        .with_children(|tree| {
                            tree.spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: color.clone(),
                                    custom_size: Some(Vec2::new(
                                        projection.right - projection.left * 5.0,
                                        v_offset,
                                    )),
                                    ..default()
                                },
                                transform: Transform::from_translation(Vec3::new(
                                    0.0,
                                    -image.size().y / 2.0 - v_offset / 2.0,
                                    0.0,
                                )),
                                ..default()
                            });
                        });
                }
            });
    }
}

#[derive(Default)]
pub struct LastPosition(pub f32);

pub fn move_parallax(
    mut parallax_query: Query<(&mut Transform, &Parallax), Without<MainCamera>>,
    camera_query: Query<&Transform, With<MainCamera>>,
    mut last_position: Local<LastPosition>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        if last_position.0 != 0.0 {
            for (mut parallax_transform, parallax) in parallax_query.iter_mut() {
                parallax_transform.translation.y -=
                    (camera_transform.translation.y - last_position.0) * parallax.0;
            }
        }
        last_position.0 = camera_transform.translation.y
    }
}
