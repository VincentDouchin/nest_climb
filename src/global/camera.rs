use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        projection: OrthographicProjection {
            scale: 0.2,
            ..default()
        },
        ..default()
    });
}

pub fn camera_plugin(app: &mut App) {
    app.add_startup_system(spawn_camera);
}
