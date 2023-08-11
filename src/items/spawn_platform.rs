use crate::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Platform;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlatformBundle {
    platform: Platform,
    #[ldtk_entity]
    pub patrol: Patrol,
}

pub fn spawn_platforms(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), Added<Platform>>,
    assets: Res<MyAssets>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.scale = Vec3::splat(1.0);
        commands.entity(entity).insert((
            AnimatedSpriteBundle::new(assets.platform.clone()),
            Collider::cuboid(24.0, 4.0),
            Velocity::default(),
            RigidBody::Dynamic,
            KinematicCharacterController::default(),
            LockedAxes::ROTATION_LOCKED,
            GhostPlatform::default(),
        ));
    }
}
