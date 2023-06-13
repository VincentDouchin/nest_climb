use crate::*;
use bevy::{prelude::*, window::*};
use bevy_ecs_ldtk::prelude::*;
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraTarget {
    pub x: bool,
    pub y: bool,
}
impl CameraTarget {
    pub fn new(x: bool, y: bool) -> Self {
        CameraTarget { x, y }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.5,
                ..default()
            },
            ..default()
        },
        MainCamera,
    ));
}

pub fn move_camera(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<CameraTarget>)>,
    target_query: Query<(&Transform, &CameraTarget), Without<MainCamera>>,
) {
    for (target_transform, camera_target) in target_query.iter() {
        for mut camera_transform in camera_query.iter_mut() {
            if camera_target.x {
                camera_transform.translation.x = target_transform.translation.x;
            }
            if camera_target.y {
                camera_transform.translation.y = target_transform.translation.y;
            }
        }
    }
}

pub fn set_camera_to_level_center(
    current_level: Res<CurrentLevel>,
    files: Res<Assets<LdtkAsset>>,
    level_selection: Res<LevelSelection>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(file) = files.get(&current_level.file) {
        if let Some(level) = file.get_level(&level_selection) {
            if let Ok((mut camera_transform, mut projection)) = camera_query.get_single_mut() {
                camera_transform.translation.x = level.px_wid as f32 / 2.0;
                if let Ok(window) = window_query.get_single() {
                    projection.scale = level.px_wid as f32 / window.width()
                }
            }
        }
    }
}

pub fn camera_plugin(app: &mut App) {
    app.add_startup_system(spawn_camera);
    app.add_system(move_camera);
    app.add_system(set_camera_to_level_center.in_schedule(OnEnter(GameState::Run)));
}
