use crate::{AnimationTimerComponent, CameraTarget, DirectionComponent, MyAssets, SpriteDirection};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct PlayerController {
    pub grounded: bool,
}
impl Default for PlayerController {
    fn default() -> Self {
        PlayerController { grounded: true }
    }
}

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
        AnimationTimerComponent::new(None),
        DirectionComponent(SpriteDirection::Right),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::cuboid(10.0, 12.0),
        Velocity::zero(),
        PlayerController::default(),
        CameraTarget,
    ));
}
