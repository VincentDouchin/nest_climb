use bevy::prelude::*;

use crate::*;

#[derive(Component)]
pub struct Background;

pub fn spawn_background(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: assets.background.clone(),
            ..default()
        },
        Background,
    ));
}

pub fn resize_background(
    mut background_query: Query<&mut Sprite, With<Background>>,
    camera_query: Query<
        &OrthographicProjection,
        (With<MainCamera>, Changed<OrthographicProjection>),
    >,
) {
    for mut background_sprite in background_query.iter_mut() {
        if let Ok(camera) = camera_query.get_single() {
            background_sprite.custom_size =
                Some(Vec2::new(camera.area.width(), camera.area.width()))
        }
    }
}
pub fn background_follow_camera(
    mut background_query: Query<&mut Transform, (With<Background>, Without<MainCamera>)>,
    camera_query: Query<&Transform, (With<MainCamera>, Without<Background>)>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        for mut background_transform in background_query.iter_mut() {
            background_transform.translation.x = camera_transform.translation.x;
            background_transform.translation.y = camera_transform.translation.y;
        }
    }
}

pub fn background_plugin(app: &mut App) {
    app.add_system(spawn_background.in_schedule(OnExit(GameState::AssetLoading)))
        .add_system(resize_background)
        .add_system(background_follow_camera);
}
