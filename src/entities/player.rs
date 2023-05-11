use crate::{AnimationTimer, MyAssets};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct PlayerController;

pub fn spawn_player(mut commands: Commands, assets: Res<MyAssets>) {
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::ZERO,
                ..default()
            },
            texture_atlas: assets.bird.clone(),
            ..default()
        },
        AnimationTimer::new(None),
        RigidBody::Dynamic,
        Collider::cuboid(16.0, 16.0),
        PlayerController,
    ));
}
