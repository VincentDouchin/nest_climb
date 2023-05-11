use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn spawn_ground(mut commands: Commands) {
    commands.spawn((
        Collider::cuboid(500.0, 50.0),
        TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)),
    ));
}
