use bevy::prelude::*;
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraTarget;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            projection: OrthographicProjection {
                scale: 0.2,
                ..default()
            },
            ..default()
        },
        LookTransformBundle {
            transform: LookTransform::new(Vec3::Z, Vec3::ZERO, Vec3::Y),
            smoother: Smoother::new(0.0),
        },
        MainCamera,
    ));
}

fn camera_follow_target(
    mut camera_query: Query<&mut LookTransform, (With<MainCamera>, Without<CameraTarget>)>,
    target_query: Query<&Transform, (With<CameraTarget>, Without<MainCamera>)>,
) {
    for target_transform in target_query.iter() {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.target = target_transform.local_y();
            camera_transform.eye = target_transform.local_y() + Vec3::Z;
        }
    }
}

pub fn camera_plugin(app: &mut App) {
    app.add_startup_system(spawn_camera);
    app.add_system(camera_follow_target);
    app.add_plugin(LookTransformPlugin);
}
