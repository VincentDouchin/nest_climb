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
    query: Query<Entity, Added<Platform>>,
    assets: Res<MyAssets>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert((
            assets.platform.clone(),
            Collider::cuboid(24.0, 4.0),
            Velocity::default(),
            RigidBody::Dynamic,
            KinematicCharacterController::default(),
            LockedAxes::ROTATION_LOCKED,
            TextureAtlasSprite::default(),
        ));
    }
}
