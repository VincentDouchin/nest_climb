use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use crate::*;

#[derive(Component)]
pub struct Background;

pub fn spawn_background(
    mut commands: Commands,
    assets: Res<MyAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = window_query.get_single() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(window.width(), window.height())),
                    ..default()
                },
                texture: assets.background.clone(),
                ..default()
            },
            Background,
        ));
    }
}
pub fn resize_background(
    events: EventReader<WindowResized>,
    mut background_query: Query<&mut Sprite, With<Background>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if events.len() > 0 {
        if let Ok(window) = window_query.get_single() {
            for mut background_sprite in background_query.iter_mut() {
                background_sprite.custom_size = Some(Vec2::new(window.width(), window.height()))
            }
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
