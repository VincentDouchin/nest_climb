use crate::*;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, window::*};
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
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::NONE),
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
    level_query: Query<&GlobalTransform, (With<Handle<LdtkLevel>>, Without<MainCamera>)>,
) {
    if let Some(file_handle) = &current_level.file {
        if let Some(file) = files.get(file_handle) {
            if let Some(level) = file.get_level(&level_selection) {
                if let Ok((mut camera_transform, mut projection)) = camera_query.get_single_mut() {
                    camera_transform.translation.x = level.px_wid as f32 / 2.0;
                    if let Ok(window) = window_query.get_single() {
                        projection.scale = level.px_wid as f32 / window.width();
                        for level_global_transform in level_query.iter() {
                            camera_transform.translation.y = camera_transform.translation.y.max(
                                level_global_transform.translation().y
                                    + window.height() / 2.0 * projection.scale,
                            )
                        }
                    }
                }
            }
        }
    }
}

pub fn camera_plugin(app: &mut App) {
    app.add_startup_system(spawn_camera);
    app.add_systems((move_camera, set_camera_to_level_center).chain());
}
