use crate::*;
use bevy::{prelude::*, window::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_pixel_camera::*;
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
#[derive(Component, Default)]
pub struct CameraBounds {
    pub top: f32,
    pub bottom: f32,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        PixelCameraBundle::from_resolution(320, 240, false),
        MainCamera,
        CameraBounds::default(),
    ));
}

pub fn move_camera(
    mut camera_query: Query<
        (&mut Transform, &CameraBounds),
        (With<MainCamera>, Without<CameraTarget>),
    >,
    target_query: Query<&Transform, (Without<MainCamera>, Changed<Transform>, With<CameraTarget>)>,
) {
    for target_transform in target_query.iter() {
        for (mut camera_transform, bounds) in camera_query.iter_mut() {
            camera_transform.translation.y = target_transform.translation.y.max(bounds.top)
        }
    }
}

pub fn set_camera_to_level_center(
    current_level: Res<CurrentLevel>,
    files: Res<Assets<LdtkAsset>>,
    level_selection: Res<LevelSelection>,
    mut camera_query: Query<
        (&mut Transform, &mut CameraBounds, &mut PixelProjection),
        With<MainCamera>,
    >,
    window_query: Query<&Window, With<PrimaryWindow>>,
    level_query: Query<&GlobalTransform, (With<Handle<LdtkLevel>>, Without<MainCamera>)>,
) {
    if let Some(file_handle) = &current_level.file {
        if let Some(file) = files.get(file_handle) {
            if let Some(level) = file.get_level(&level_selection) {
                if let Ok((mut camera_transform, mut camera_bounds, mut projection)) =
                    camera_query.get_single_mut()
                {
                    camera_transform.translation.x = level.px_wid as f32 / 2.0;
                    if let Ok(window) = window_query.get_single() {
                        projection.desired_width = Some(level.px_wid);
                        projection.desired_height =
                            Some(level.px_wid * (window.height() / window.width()) as i32);
                        let zoom = window.height() / window.width();
                        for level_global_transform in level_query.iter() {
                            camera_bounds.top = level_global_transform.translation().y
                                + window.height() / 2.0 * zoom as f32;
                            camera_bounds.bottom = level_global_transform.translation().y
                                - window.height() / 2.0 * zoom as f32;
                        }
                    }
                }
            }
        }
    }
}

pub fn camera_plugin(app: &mut App) {
    app.add_systems(OnExit(GameState::AssetLoading), spawn_camera);
    app.add_systems(Update, (move_camera, set_camera_to_level_center).chain());
}
