use bevy::prelude::*;
use bevy_pixel_camera::PixelProjection;

use crate::*;

#[derive(Component)]
pub struct Background;

#[derive(Component, Default)]
pub struct FollowCamera {
    pub offset: Vec2,
}

pub fn spawn_background(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: assets.background.clone(),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        Background,
        FollowCamera::default(),
    ));
}

pub fn resize_background(
    mut background_query: Query<&mut Sprite, With<Background>>,
    camera_query: Query<&PixelProjection, (With<MainCamera>, Changed<PixelProjection>)>,
) {
    for mut background_sprite in background_query.iter_mut() {
        if let Ok(camera) = camera_query.get_single() {
            background_sprite.custom_size = Some(Vec2::new(
                camera.right - camera.left,
                camera.top - camera.bottom,
            ))
        }
    }
}
pub fn background_follow_camera(
    mut background_query: Query<(&mut Transform, &FollowCamera), Without<MainCamera>>,
    camera_query: Query<(&Transform, &PixelProjection), (With<MainCamera>, Without<FollowCamera>)>,
) {
    if let Ok((camera_transform, projection)) = camera_query.get_single() {
        for (mut background_transform, follow_camera) in background_query.iter_mut() {
            background_transform.translation.x = camera_transform.translation.x
                + (projection.right - projection.left) * follow_camera.offset.x;
            background_transform.translation.y = camera_transform.translation.y
                + (projection.top - projection.bottom) * follow_camera.offset.y;
        }
    }
}

pub fn background_plugin(app: &mut App) {
    app.add_systems(OnExit(GameState::AssetLoading), spawn_background)
        .add_systems(Update, resize_background)
        .add_systems(Update, background_follow_camera);
}
