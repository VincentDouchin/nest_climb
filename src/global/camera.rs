use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraTarget;

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

fn camera_follow_target(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<CameraTarget>)>,
    target_query: Query<&Transform, (With<CameraTarget>, Without<MainCamera>)>,
) {
    for target_transform in target_query.iter() {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation.y = target_transform.translation.y;
            camera_transform.translation.x = target_transform.translation.x;
        }
    }
}

pub fn camera_plugin(app: &mut App) {
    app.add_startup_system(spawn_camera);
    app.add_system(camera_follow_target);
}
